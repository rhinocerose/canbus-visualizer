## Setting up Virtual CAN Bus
```
sudo modprobe vcan
sudo ip link add vcan0 type vcan
sudo ip link set vcan0 up
```

## Masking for Relevant Frames
```rust
if let Some(signal_lookup) = signal_lookup.as_ref() {
    if ((frame.id() << 16) >> 16) == 52456 {
        print_dbc_signals(signal_lookup, &frame);
    }
}
```
