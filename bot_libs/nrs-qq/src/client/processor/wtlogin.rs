use alloc::sync::Arc;

use crate::engine::command::wtlogin::*;
use crate::handler::QEvent;
use crate::provider::*;

impl<CP, OSCP, RP, MP, TP, TCP> crate::Client<CP, OSCP, RP, MP, TP, TCP>
where
    CP: ChannelProvider + 'static,
    OSCP: OneShotChannelProvider + 'static,
    RP: RwLockProvider + 'static,
    TP: TaskProvider + 'static,
    TCP: TcpStreamProvider + 'static,
    MP: MutexProvider + 'static,
{
    pub(crate) async fn process_login_response(self: &Arc<Self>, login_response: LoginResponse) {
        if let LoginResponse::Success(ref success) = login_response {
            if let Some(info) = success.account_info.clone() {
                let mut account_info = self.account_info.write().await;
                account_info.nickname = info.nick;
                account_info.age = info.age;
                account_info.gender = info.gender;
            }
        }
        self.engine
            .write()
            .await
            .process_login_response(login_response);
        self.handler.handle(QEvent::Login(self.uin().await)).await;
    }

    pub(crate) async fn process_trans_emp_response(&self, qrcode_state: QRCodeState) {
        if let QRCodeState::Confirmed(resp) = qrcode_state {
            self.engine.write().await.process_qrcode_confirmed(resp);
        }
    }
}
