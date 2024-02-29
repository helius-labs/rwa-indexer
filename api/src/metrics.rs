use cadence_macros::is_global_default_set;
use {
    crate::config::Config,
    cadence::{BufferedUdpMetricSink, QueuingMetricSink, StatsdClient},
    cadence_macros::set_global_default,
    std::net::UdpSocket,
};

pub fn safe_metric<F: Fn()>(f: F) {
    if is_global_default_set() {
        f()
    }
}

#[macro_export]
macro_rules! metric {
    {$($block:stmt;)*} => {
        if is_global_default_set() {
            $(
                $block
            )*
        }
    };
}

pub fn setup_metrics(config: &Config) {
    let uri = config.metrics_host.clone();
    let port = config.metrics_port;
    let env = config.env.clone().unwrap_or_else(|| "dev".to_string());
    if uri.is_some() || port.is_some() {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        socket.set_nonblocking(true).unwrap();
        let host = (uri.unwrap(), port.unwrap());
        let udp_sink = BufferedUdpMetricSink::from(host, socket).unwrap();
        let queuing_sink = QueuingMetricSink::from(udp_sink);
        let builder = StatsdClient::builder("rapi", queuing_sink);
        let client = builder.with_tag("env", env).build();
        set_global_default(client);
    }
}
