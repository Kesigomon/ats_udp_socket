use std::net::UdpSocket;

fn main(){
    let socket = UdpSocket::bind("127.0.0.1:5901").unwrap();
    let mut buf = [0u8; 256];
    let mut before_speed = 0.0;
    let mut before_time = 0;
    loop {
        let size = socket.recv(&mut buf).unwrap();
        let speed = f32::from_be_bytes(buf[0..4].try_into().unwrap()) as f64;
        let speed_ms = 5.0 * speed / 18.0;
        let time = i32::from_be_bytes(buf[4..8].try_into().unwrap());
        let seconds = (time / 1000) % 60;
        let minutes = (time / 60000) % 60;
        let hours = (time / 3600000);
        let accelarete= 2500.0 * (speed - before_speed) / ((time - before_time) as f64) / 9.0;
        print!("{:.0}[km/h] {:.3}[m/s]", speed, accelarete);
        if accelarete < 0.0{
            print!(" {:.3}[m]", -(speed_ms * speed_ms) / accelarete / 2.0);
        }
        println!();
        before_speed = speed;
        before_time = time;

    }
}