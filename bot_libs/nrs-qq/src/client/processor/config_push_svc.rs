use crate::provider::*;
use crate::Client;
use crate::RQResult;
use alloc::vec::Vec;
use bytes::Bytes;
use no_std_net::{Ipv4Addr, SocketAddr};
use nrq_engine::{command::config_push_svc::*, common::RQIP};
impl<CP, OSCP, RP, MP, TP, TCP> Client<CP, OSCP, RP, MP, TP, TCP>
where
    CP: ChannelProvider,
    OSCP: OneShotChannelProvider,
    RP: RwLockProvider,
    TP: TaskProvider,
    TCP: TcpStreamProvider,
    MP: MutexProvider,
{
    pub(crate) async fn process_config_push_req(
        &self,
        config_push_req: ConfigPushReq,
    ) -> RQResult<()> {
        // send response to server
        let resp = config_push_req.resp;
        let response = self.engine.read().await.build_conf_push_resp_packet(
            resp.t,
            resp.pkt_seq,
            resp.jce_buf,
        );
        self.send(response).await?;
        match config_push_req.body {
            ConfigPushBody::Unknown => {}
            ConfigPushBody::SsoServers { .. } => {}
            ConfigPushBody::FileStorageInfo { info: _, rsp_body } => {
                let mut session = self.highway_session.write().await;
                if let Some(rsp_body) = rsp_body {
                    session.sig_session = Bytes::from(rsp_body.sig_session.unwrap_or_default());
                    session.session_key = Bytes::from(rsp_body.session_key.unwrap_or_default());
                    session.uin = self.uin().await;
                    session.app_id = self.engine.read().await.transport.version.app_id as i32;
                    let mut highway_addrs = self.highway_addrs.write().await;
                    rsp_body.addrs.into_iter().for_each(|addr| {
                        let service_type = addr.service_type.unwrap_or_default();
                        if service_type == 10 {
                            let addrs: Vec<SocketAddr> = addr
                                .addrs
                                .into_iter()
                                .map(|addr| {
                                    SocketAddr::new(
                                        Ipv4Addr::from(RQIP(addr.ip.unwrap_or_default())).into(),
                                        addr.port.unwrap_or_default() as u16,
                                    )
                                })
                                .collect();
                            highway_addrs.extend(addrs);
                        } else if service_type == 11 {
                            // TODO
                        }
                    })
                }
            }
        }
        // TODO process
        Ok(())
    }
}
