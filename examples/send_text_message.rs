use tibco_ems_sys::*;
use std::ffi::CString;

fn main() {
  unsafe{
    let factory = tibemsConnectionFactory_Create();

    let url = "tcp://localhost:7222";
    let user="admin";
    let password="admin";

    let c_url = CString::new(url).unwrap();
    let status = tibemsConnectionFactory_SetServerURL(factory, c_url.as_ptr());
    println!("tibemsConnectionFactory_SetServerURL: {:?}",status);

    let mut conn: usize = 0;
    let c_user = CString::new(user).unwrap();
    let c_password = CString::new(password).unwrap();
    let status = tibemsConnectionFactory_CreateConnection(factory,&mut conn,c_user.as_ptr(),c_password.as_ptr());
    println!("tibemsConnectionFactory_CreateConnection: {:?}",status);

    let mut sess: usize = 0;
    let status = tibemsConnection_CreateSession(conn,&mut sess,tibems_bool::TIBEMS_FALSE,tibemsAcknowledgeMode::TIBEMS_AUTO_ACKNOWLEDGE);
    println!("tibemsConnection_CreateSession: {:?}",status);

    let dest_str = "myqueue";
    let mut dest: usize = 0;
    let c_dest = CString::new(dest_str).unwrap();
    let status = tibemsDestination_Create(&mut dest, tibemsDestinationType::TIBEMS_QUEUE, c_dest.as_ptr());
    println!("tibemsDestination_Create: {:?}",status);

    let mut producer: usize = 0;
    let status = tibemsSession_CreateProducer(sess,&mut producer,dest);
    println!("tibemsSession_CreateProducer: {:?}",status);

    let mut msg: usize = 0;
    let status = tibemsTextMsg_Create(&mut msg);
    println!("tibemsTextMsg_Create: {:?}",status);

    let content="hallo welt";
    let c_content = CString::new(content).unwrap();
    let status = tibemsTextMsg_SetText(msg,c_content.as_ptr());
    println!("tibemsTextMsg_SetText: {:?}",status);

    let c_key = CString::new("key").unwrap();
    let c_val = CString::new("val").unwrap();
    let status = tibemsMsg_SetStringProperty(msg,c_key.as_ptr(),c_val.as_ptr());
    println!("tibemsMsg_SetStringProperty: {:?}",status);

    let status = tibemsMsgProducer_Send(producer, msg);
    println!("tibemsMsgProducer_Send: {:?}",status);
  }
}
