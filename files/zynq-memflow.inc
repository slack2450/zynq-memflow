RUST_VERSION = "1.67.1"

do_install() {
	     install -d ${D}${bindir}
	     install -m 0755 ./target/arm-xilinx-linux-gnueabi/release/zynq-memflow ${D}${bindir}
}