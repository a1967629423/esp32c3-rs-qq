use alloc::string::String;
use alloc::sync::Arc;
use core::fmt::Debug;
use core::time::Duration;

use bytes::Bytes;

use nrq_engine::command::wtlogin::LoginResponse;

use crate::ext::common::after_login;
use crate::{Client, RQError, RQResult};

use crate::provider::*;

pub trait Connector<CP, OSCP, RP, MP, TP, TCP>
where
    CP: ChannelProvider + 'static,
    OSCP: OneShotChannelProvider + 'static,
    RP: RwLockProvider + 'static,
    TP: TaskProvider + 'static,
    TCP: TcpStreamProvider + 'static,
    MP: MutexProvider + 'static,
{
    type ConnectError: Debug = ();
    type ConnectFuture: futures::Future<Output = Result<TCP, Self::ConnectError>> + Send;
    fn connect(&self, client: &Arc<Client<CP, OSCP, RP, MP, TP, TCP>>) -> Self::ConnectFuture;
}

pub struct DefaultConnector;

impl<CP, OSCP, RP, MP, TP, TCP> Connector<CP, OSCP, RP, MP, TP, TCP> for DefaultConnector
where
    CP: ChannelProvider + 'static,
    OSCP: OneShotChannelProvider + 'static,
    RP: RwLockProvider + 'static,
    TP: TaskProvider + 'static,
    TCP: TcpStreamProvider + 'static,
    MP: MutexProvider + 'static,
{
    type ConnectError = <TCP as TcpStreamProvider>::IOError;
    type ConnectFuture = impl futures::Future<Output = Result<TCP, Self::ConnectError>> + Send;
    fn connect(&self, client: &Arc<Client<CP, OSCP, RP, MP, TP, TCP>>) -> Self::ConnectFuture {
        let c = client.clone();
        async move { TCP::connect(c.get_address()).await }
    }
}

/// 自动重连，在掉线后使用，会阻塞到重连结束
pub async fn auto_reconnect<
    CP,
    OSCP,
    RP,
    MP,
    TP,
    TCP,
    C: Connector<CP, OSCP, RP, MP, TP, TCP> + Sync,
>(
    client: Arc<Client<CP, OSCP, RP, MP, TP, TCP>>,
    credential: Credential,
    interval: Duration,
    max: usize,
    connector: C,
) where
    CP: ChannelProvider + 'static,
    OSCP: OneShotChannelProvider + 'static,
    RP: RwLockProvider + 'static,
    TP: TaskProvider + 'static,
    TCP: TcpStreamProvider + 'static,
    MP: MutexProvider + 'static,
{
    let mut count = 0;
    loop {
        client.stop();
        tracing::error!("client will reconnect after {} seconds", interval.as_secs());
        TP::sleep(interval).await;
        let stream = if let Ok(stream) = connector.connect(&client).await {
            count = 0;
            stream
        } else {
            count += 1;
            if count > max {
                tracing::error!("reconnect_count: {}, break!", count);
                break;
            }
            continue;
        };
        let c = client.clone();
        let handle = TP::spawn(async move { c.start(stream).await });
        TP::yield_now().await; // 等一下，确保连上了
        if let Err(err) = fast_login(client.clone(), credential.clone()).await {
            // token 可能过期了
            tracing::error!("failed to fast_login: {}, break!", err);
            break;
        }
        tracing::info!("succeed to reconnect");
        after_login(&client).await;
        handle.await.ok();
    }
}

pub trait FastLogin<CP, OSCP, RP, MP, TP, TCP>
where
    CP: ChannelProvider + 'static,
    OSCP: OneShotChannelProvider + 'static,
    RP: RwLockProvider + 'static,
    TP: TaskProvider + 'static,
    TCP: TcpStreamProvider + 'static,
    MP: MutexProvider + 'static,
{
    type LoginFuture: futures::Future<Output = RQResult<()>> + Send;
    fn fast_login(&self, client: Arc<Client<CP, OSCP, RP, MP, TP, TCP>>) -> Self::LoginFuture;
}
#[derive(Clone)]
pub struct Token(pub Bytes);
#[derive(Clone)]
pub struct Password {
    pub uin: i64,
    pub password: String,
}
#[derive(Clone)]
pub enum Credential {
    Token(Token),
    Password(Password),
    Both(Token, Password),
}

impl<CP, OSCP, RP, MP, TP, TCP> FastLogin<CP, OSCP, RP, MP, TP, TCP> for Token
where
    CP: ChannelProvider + 'static,
    OSCP: OneShotChannelProvider + 'static,
    RP: RwLockProvider + 'static,
    TP: TaskProvider + 'static,
    TCP: TcpStreamProvider + 'static,
    MP: MutexProvider + 'static,
{
    type LoginFuture = impl futures::Future<Output = RQResult<()>> + Send;
    fn fast_login(&self, client: Arc<Client<CP, OSCP, RP, MP, TP, TCP>>) -> Self::LoginFuture {
        let s = self.clone();
        async move { client.token_login(s.0.clone()).await }
    }
}
impl<CP, OSCP, RP, MP, TP, TCP> FastLogin<CP, OSCP, RP, MP, TP, TCP> for Password
where
    CP: ChannelProvider + 'static,
    OSCP: OneShotChannelProvider + 'static,
    RP: RwLockProvider + 'static,
    TP: TaskProvider + 'static,
    TCP: TcpStreamProvider + 'static,
    MP: MutexProvider + 'static,
{
    type LoginFuture = impl futures::Future<Output = RQResult<()>> + Send;
    fn fast_login(&self, client: Arc<Client<CP, OSCP, RP, MP, TP, TCP>>) -> Self::LoginFuture {
        let s = self.clone();
        async move {
            let resp = client.password_login(s.uin, &s.password).await?;
            match resp {
                LoginResponse::Success { .. } => return Ok(()),
                LoginResponse::DeviceLockLogin { .. } => {
                    return if let LoginResponse::Success { .. } = client.device_lock_login().await?
                    {
                        Ok(())
                    } else {
                        Err(RQError::Other("failed to login".into()))
                    };
                }
                _ => return Err(RQError::Other("failed to login".into())),
            }
        }
    }
}

pub async fn fast_login<CP, OSCP, RP, MP, TP, TCP>(
    client: Arc<Client<CP, OSCP, RP, MP, TP, TCP>>,
    credential: Credential,
) -> RQResult<()>
where
    CP: ChannelProvider + 'static,
    OSCP: OneShotChannelProvider + 'static,
    RP: RwLockProvider + 'static,
    TP: TaskProvider + 'static,
    TCP: TcpStreamProvider + 'static,
    MP: MutexProvider + 'static,
{
    return match credential {
        Credential::Token(token) => token.fast_login(client).await,
        Credential::Password(password) => password.fast_login(client).await,
        Credential::Both(token, password) => match token.fast_login(client.clone()).await {
            Ok(_) => Ok(()),
            Err(_) => password.fast_login(client).await,
        },
    };
}
