use crate::input::Input;

pub struct TotalConnections {
    bl_conn: Vec<Connection>,
    wl_conn: Vec<Connection>,
    bl_tot: u64,
    wl_tot: u64
}

impl TotalConnections {

    pub fn new() -> TotalConnections {
        TotalConnections {
            bl_conn: vec![],
            wl_conn: vec![],
            bl_tot: 0,
            wl_tot: 0
        }
    }

    pub fn get_bl(self) -> Vec<Connection> {
        self.bl_conn
    }

    pub fn push(&mut self, conn: Connection) {
        if !self.bl_conn.contains(&conn) & !self.wl_conn.contains(&conn) {
            self.bl_tot += 1;
            self.bl_conn.push(conn);
        }
    }

    pub fn try_push(&mut self, conn: Connection, input: &Input) {
        if !self.bl_conn.contains(&conn) & !self.wl_conn.contains(&conn) {
            self.bl_tot += 1;

            let conn_clone = conn.clone();
            let conn_clone_ip = conn_clone.get_ip();
            println!("[+] {} » {}", self.bl_tot, conn_clone_ip);

            let ipt = iptables::new(false).unwrap();
            ipt.insert(input.get_table(), input.get_chain(), format!("-s {} -j DROP", conn_clone_ip).as_str(), 1).unwrap();

            self.bl_conn.push(conn);
        }
    }

    pub fn push_wl(&mut self, conn: Connection) {
        if !self.wl_conn.contains(&conn) {
            self.wl_tot += 1;
            println!("[/] {} » {}", self.wl_tot, &conn.clone().get_ip());
            self.wl_conn.push(conn);
        }
    }

}

#[derive(Debug, PartialEq, Clone)]
pub struct Connection(String);

impl Connection {
    
    pub fn new(ip: String) -> Connection {
        Connection(ip)
    }

    pub fn get_ip(self) -> String {
        self.0
    }

}