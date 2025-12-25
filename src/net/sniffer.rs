// sniffer.rs
mod sniffer;
// This module is responsible for capturing and processing network packets.
// It interfaces with the underlying network stack to gather data for analysis.
// It provides functionality to filter packets, extract relevant information,
// and pass the data to the analyzer for further processing.
// It also handles the configuration of packet capture parameters and manages
// the lifecycle of the sniffer, including starting and stopping the capture process.
