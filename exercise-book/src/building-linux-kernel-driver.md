# Building a Linux Kernel Driver using Rust

In this example we're going to build a very basic Linux Kernel driver in Rust,
compile a Linux kernel, and load the driver into it.

You will need QEMU installed and in your `$PATH`. If you have an AArch64 machine
(Mac with Apple Silicon, Raspberry Pi 3 or newer, etc) you should use
`qemu-system-aarch64`. If you have an x86-64 machine, you should use
`qemu-system-x86_64`. You can use the 'wrong' one - it'll work just fine, but it
will go much more slowly, and we have a lot of Linux kernel to build.

## Task 1 - Fetch a Debian disk image

This example uses Debian Stable 'nocloud' disk image. Other disk images might
work but this is the one we tested.

Make yourself a work area on your machine, and download either <https://cdimage.debian.org/images/cloud/bookworm/20250316-2053/debian-12-nocloud-arm64-20250316-2053.qcow2> (AArch64 aka Arm64) or <https://cdimage.debian.org/images/cloud/bookworm/20250316-2053/debian-12-nocloud-amd64-20250316-2053.qcow2> (x86-64 aka AMD64)

For example:

```bash
mkdir linux-rust-demo
cd linux-rust-demo
wget https://cdimage.debian.org/images/cloud/bookworm/20250316-2053/debian-12-nocloud-arm64-20250316-2053.qcow2
```

(Windows users can use their favourite tool to make the folder, and a web browser to download the disk image)

## Task 1a - Fetch the BIOS (AArch64 only)

If you are going to use AArch64, you'll need a UEFI boot-loader because QEMU
doesn't come with one (or at least, the QEMU in homebrew that I used didn't come
with one).

Download it from <https://gist.githubusercontent.com/theboreddev/5f79f86a0f163e4a1f9df919da5eea20/raw/f546faea68f4149c06cca88fa67ace07a3758268/QEMU_EFI-a096471-edk2-stable202011.tar.gz> and unpack it.

```bash
wget https://gist.githubusercontent.com/theboreddev/5f79f86a0f163e4a1f9df919da5eea20/raw/f546faea68f4149c06cca88fa67ace07a3758268/QEMU_EFI-a096471-edk2-stable202011.tar.gz
tar xvf QEMU_EFI-a096471-edk2-stable202011.tar.gz
```

(Windows users, use your favourite tools for this)

## Task 2 - Resize the disk image

If your downloads take a while, you may want to make a backup copy of the disk
image, because as soon as you boot up the VM, the disk will be changed.

Then we're going to resize the disk image because it's too small (this applies
to both the AArch64 one and the x86-64 one). We'll deal with making the
partition larger a bit later once the VM has booted.

```bash
qemu-img resize debian-12-nocloud-arm64-20250316-2053.qcow2 +32G
```

## Task 3 - Boot it up

We're now going to boot the VM.

For AArch64 on Apple Silicon macOS:

```bash
qemu-system-aarch64 -m 8G -M virt -cpu host -accel hvf -smp 8 -bios QEMU_EFI.fd -drive if=none,file=debian-12-nocloud-arm64-20250316-2053.qcow2,id=hd0 -device virtio-blk-device,drive=hd0 -device e1000,netdev=net0 -netdev user,id=net0,hostfwd=tcp:127.0.0.1:5555-:22 -nographic -serial telnet:localhost:4321,server,wait
```

For AArch64 on Arm Linux:

```bash
qemu-system-aarch64 -m 8G -M virt -cpu host -accel kvm -smp 8 -bios QEMU_EFI.fd -drive if=none,file=debian-12-nocloud-arm64-20250316-2053.qcow2,id=hd0 -device virtio-blk-device,drive=hd0 -device e1000,netdev=net0 -netdev user,id=net0,hostfwd=tcp:127.0.0.1:5555-:22 -nographic -serial telnet:localhost:4321,server,wait
```

AArch64 otherwise:

```bash
qemu-system-aarch64 -m 8G -M virt -cpu cortex-a53 -smp 8 -bios QEMU_EFI.fd -drive if=none,file=debian-12-nocloud-arm64-20250316-2053.qcow2,id=hd0 -device virtio-blk-device,drive=hd0 -device e1000,netdev=net0 -netdev user,id=net0,hostfwd=tcp:127.0.0.1:5555-:22 -nographic -serial telnet:localhost:4321,server,wait
```

For x86-64 on x86-64 Windows:

```bash
qemu-system-x86_64 -m 8G -M q35 -accel whpx -smp 8 -hda debian-12-nocloud-amd64-20250316-2053.qcow2 -device e1000,netdev=net0 -netdev user,id=net0,hostfwd=tcp:127.0.0.1:5555-:22 -nographic -serial telnet:localhost:4321,server,wait
```

For x86-64 on x86-64 Linux:

```bash
qemu-system-x86_64 -m 8G -M q35 -accel kvm -smp 8 -hda debian-12-nocloud-amd64-20250316-2053.qcow2 -device e1000,netdev=net0 -netdev user,id=net0,hostfwd=tcp:127.0.0.1:5555-:22 -nographic -serial telnet:localhost:4321,server,wait
```

x86-64 otherwise:

```bash
qemu-system-x86_64 -m 8G -M q35 -smp 8 -hda debian-12-nocloud-amd64-20250316-2053.qcow2 -device e1000,netdev=net0 -netdev user,id=net0,hostfwd=tcp:127.0.0.1:5555-:22 -nographic -serial telnet:localhost:4321,server,wait
```

In all cases I gave the machine 8 GiB of RAM and 8 CPU cores. You may want to
tweak that to suit your needs.

Connect to your virtual machine over Telnet (you can use PuTTY, or your
favourite telnet client) on `localhost:4321` You should end up at a login
prompt. The user is `root` and there is no password.

## Task 4 - Resize the partition

The Debian image we downloaded has quite a small root partition, and we're going
to need a lot more space to build the Linux kernel. We already made the virtual
disk (the QCOW file) larger, so now let's go in and resize it the partition to
use the extra space.

Inside your VM, run:

```bash
apt update
apt install fdisk
cfdisk /dev/*da
```

Once in `cfdisk`, use the arrow keys to select `[ Sort ]`, then select the root
filesystem (the bottom one) and pick `[ Resize ]`, then `[ Write ]`, and
`[ Quit ]`. You'll need to type `yes` in response to `[ Write ]`.

Now run `reboot` and connect to the VM again. Running `df -h` should show that
`/` is about 35 GiB in size - Debian's start-up scripts automatically resized
the root filesystem to fill our newly enlarged the partition.

## Task 5 - Pre-requisites

If you have an SSH key, you can install it now and start an SSH server.

```bash
apt install openssh-server
nano .ssh/authorized_keys # paste your key into this file
```

If you do that, you can SSH into the VM using `localhost:5555` right away. You
might prefer that to whatever telnet client you were using. Or, keep using
telnet - either is fine.

Now install some more tools:

```bash
apt install build-essential libssl-dev python3 flex bison bc libncurses-dev gawk openssl libssl-dev libelf-dev libudev-dev libpci-dev libiberty-dev autoconf llvm clang lld git 
curl https://sh.rustup.rs | bash
source $HOME/.cargo/env
cargo install --locked bindgen-cli
```

## Task 6 - Build a kernel

Let's grab Linux 6.14 and build it.

```bash
curl -O https://cdn.kernel.org/pub/linux/kernel/v6.x/linux-6.14.tar.xz
tar xvf linux-6.14.tar.xz
cd linux-6.14
make LLVM=1 rustavailable
make LLVM=1 defconfig 
make LLVM=1 menuconfig  # General setup / [*] Rust support
```

In the `menuconfig` interface you will need to enter *General Support* and
scroll down to *Rust support*. Press `Y` to enable Rust support. Then use the
arrow keys to select `< Exit >` and `< Exit >` again, then `< Yes >` to save.

Now we can build and install our kernel.

```bash
make LLVM=1 -j8
make LLVM=1 modules_install
make LLVM=1 install
reboot
```

This takes around 12 minutes or so on a fast laptop (using Hypervisor
acceleration). It'll be *much* longer if you're on the 'opposite' architecture
and are having to fully emulate the guest processor.

## Task 7 - Build a kernel module

OK! Now we have Linux 6.14 with Rust support enabled. Let's build an out-of-tree kernel module.

```bash
git clone https://github.com/Rust-for-Linux/rust-out-of-tree-module
cd rust-out-of-tree-module
git checkout 15de8569df46e16f4940b52c91ee8f6bfbe5ab22
make KDIR=../linux-6.14 LLVM=1
```

The kernel module has been compiled as `rust_out_of_tree.ko`. Let's load it.

```bash
insmod ./rust_out_of_tree.ko
```

As it loaded, you should see a message from the kernel - if you're on SSH, check
the telnet window. If you don't have telnet connected, you can review the kernel
logs with `dmesg`.

```console
root@localhost:~/rust-out-of-tree-module# insmod rust_out_of_tree.ko
[   13.933513] rust_out_of_tree: loading out-of-tree module taints kernel.
[   13.938155] rust_out_of_tree: Rust out-of-tree sample (init)
root@localhost:~/rust-out-of-tree-module# dmesg
...
[   13.933513] rust_out_of_tree: loading out-of-tree module taints kernel.
[   13.938155] rust_out_of_tree: Rust out-of-tree sample (init)
root@localhost:~/rust-out-of-tree-module#
```

Now let's unload it.

```bash
rmmod rust_out_of_tree
```

Again, you should see some output:

```console
root@localhost:~/rust-out-of-tree-module# rmmod rust_out_of_tree
[   72.677287] rust_out_of_tree: My numbers are [72, 108, 200]
[   72.677835] rust_out_of_tree: Rust out-of-tree sample (exit)
root@localhost:~/rust-out-of-tree-module# dmesg
...
[   13.933513] rust_out_of_tree: loading out-of-tree module taints kernel.
[   13.938155] rust_out_of_tree: Rust out-of-tree sample (init)
[   72.677287] rust_out_of_tree: My numbers are [72, 108, 200]
[   72.677835] rust_out_of_tree: Rust out-of-tree sample (exit)
root@localhost:~/rust-out-of-tree-module#
```

## Task 8 - Create a device

As of Linux 6.14, Rust for Linux has Rust APIs for:

* Block Devices
* Miscellaneous Devices
* Network Devices

A *Miscellaneous Device* has an entry like `/dev/foobar` and we can open it,
close it, and send it `ioctl` requests.

Looking at the documentation for the [`MiscDevice::register`] method we can see
that we get an opaque object that implements [`PinInit`] rather than a concrete
type. So, we're going to need to make a bunch of changes, step by step.

[`MiscDevice::register`]: https://rust.docs.kernel.org/kernel/miscdevice/struct.MiscDeviceRegistration.html#method.register
[`PinInit`]: https://rust.docs.kernel.org/kernel/init/trait.PinInit.html

First, let's remove that example `Vec` and hold `MiscDeviceRegistration`
instead. We mark the struct with `#[pin_data(PinnedDrop)]` to promise that we're
not going to be moving things around in memory whilst the module is loaded, and
mark the `_miscdev` field with `#[pin]`:

```rust ignore
#[pin_data(PinnedDrop)]
struct RustOutOfTree {
    #[pin]
    _miscdev: kernel::miscdevice::MiscDeviceRegistration<RustOutOfTreeDevice>,
}
```

Now instead of implementing `kernel::Module`, let's implement `kernel::InPlaceModule`:

```rust ignore
impl kernel::InPlaceModule for RustOutOfTree {
    fn init(_module: &'static ThisModule) -> impl PinInit<Self, Error> {
        pr_info!("Rust out-of-tree sample (init)\n");

        let options = kernel::miscdevice::MiscDeviceOptions {
            name: kernel::c_str!("rust-misc-device"),
        };

        try_pin_init!(Self {
            _miscdev <- kernel::miscdevice::MiscDeviceRegistration::register(options),
        })
    }
}
```

Instead of a plain `Result` we're returning something that implements `PinInit`.
The `try_pin_init!` macro will do this for us. The `name` in our
`kernel::miscdevice::MiscDeviceOptions` sets the name of our device in `/dev`.
Pick something else if you like!

We need to adjust our `Drop` impl, to deal with our newly pinned data structure.

```rust ignore
#[pinned_drop]
impl PinnedDrop for RustOutOfTree {
    fn drop(self: Pin<&mut Self>) {
        pr_info!("Rust out-of-tree sample (exit)\n");
    }
}
```

(We've also removed the bit that prints the `Vec` we removed).

Finally, let's make the `RustOutOfTreeDevice` we referenced earlier in our
`RustOutOfTree` structure. It's as basic as we can get away with.

```rust ignore
struct RustOutOfTreeDevice {}

#[vtable]
impl kernel::miscdevice::MiscDevice for RustOutOfTreeDevice {
    type Ptr = Pin<KBox<Self>>;

    fn open(
        _file: &kernel::fs::File,
        _misc: &kernel::miscdevice::MiscDeviceRegistration<Self>,
    ) -> Result<Pin<KBox<Self>>> {
        return Err(ENOTTY);
    }
}
```

<details>
<summary>Our full file looks like this</summary>

```rust ignore
// SPDX-License-Identifier: GPL-2.0

//! Rust out-of-tree sample

use kernel::prelude::*;

module! {
    type: RustOutOfTree,
    name: "rust_out_of_tree",
    author: "Rust for Linux Contributors",
    description: "Rust out-of-tree sample",
    license: "GPL",
}

#[pin_data(PinnedDrop)]
struct RustOutOfTree {
    #[pin]
    _miscdev: kernel::miscdevice::MiscDeviceRegistration<RustOutOfTreeDevice>,
}

impl kernel::InPlaceModule for RustOutOfTree {
    fn init(_module: &'static ThisModule) -> impl PinInit<Self, Error> {
        pr_info!("Rust out-of-tree sample (init)\n");

        let options = kernel::miscdevice::MiscDeviceOptions {
            name: kernel::c_str!("rust-misc-device"),
        };

        try_pin_init!(Self {
            _miscdev <- kernel::miscdevice::MiscDeviceRegistration::register(options),
        })
    }
}

#[pinned_drop]
impl PinnedDrop for RustOutOfTree {
    fn drop(self: Pin<&mut Self>) {
        pr_info!("Rust out-of-tree sample (exit)\n");
    }
}

struct RustOutOfTreeDevice {}

#[vtable]
impl kernel::miscdevice::MiscDevice for RustOutOfTreeDevice {
    type Ptr = Pin<KBox<Self>>;

    fn open(
        _file: &kernel::fs::File,
        _misc: &kernel::miscdevice::MiscDeviceRegistration<Self>,
    ) -> Result<Pin<KBox<Self>>> {
        return Err(ENOTTY);
    }
}
```

</details>

Let's load it and see if we get a device:

```console
$ make KDIR=../linux-6.14 LLVM=1
$ insmod ./rust_out_of_tree.ko
[ 2337.507487] rust_out_of_tree: Rust out-of-tree sample (init)
$ ls /dev/rust*
crw------- 1 root root 10, 124 Apr  2 16:29 /dev/rust-misc-device
$ rmmod rust_out_of_tree
[ 2345.938810] rust_out_of_tree: Rust out-of-tree sample (exit)
```

Nice, we got a device!

## Task 9 - Implement `open`

Let's implement the `open` function for our `RustOutOfTreeDevice`.

Our `RustOutOfTreeDevice` will need to hold onto a reference to our open `Device`:

```rust ignore
#[pin_data]
struct RustOutOfTreeDevice {
    dev: kernel::types::ARef<kernel::device::Device>,
}
```

Yes, more pinning was required. The clue was the return type of the `open` function in the `MiscDevice` trait: `Result<Pin<KBox<Self>>>`.

Let's re-write that `open` function to actually open our device.

```rust ignore
#[vtable]
impl kernel::miscdevice::MiscDevice for RustOutOfTreeDevice {
    type Ptr = Pin<KBox<Self>>;

    fn open(
        file: &kernel::fs::File,
        misc: &kernel::miscdevice::MiscDeviceRegistration<Self>,
    ) -> Result<Pin<KBox<Self>>> {
        let dev = kernel::types::ARef::from(misc.device());

        dev_info!(
            dev,
            "Opening Rust Misc Device Sample (uid = {})\n",
            file.cred().euid().into_uid_in_current_ns()
        );

        KBox::try_pin_init(
            try_pin_init! {
                RustOutOfTreeDevice {
                    dev: dev,
                }
            },
            GFP_KERNEL,
        )
    }
}
```

<details>
<summary>Our full file looks like this:</summary>

```rust ignore
// SPDX-License-Identifier: GPL-2.0

//! Rust out-of-tree sample

use kernel::prelude::*;

module! {
    type: RustOutOfTree,
    name: "rust_out_of_tree",
    author: "Rust for Linux Contributors",
    description: "Rust out-of-tree sample",
    license: "GPL",
}

#[pin_data(PinnedDrop)]
struct RustOutOfTree {
    #[pin]
    _miscdev: kernel::miscdevice::MiscDeviceRegistration<RustOutOfTreeDevice>,
}

impl kernel::InPlaceModule for RustOutOfTree {
    fn init(_module: &'static ThisModule) -> impl PinInit<Self, Error> {
        pr_info!("Rust out-of-tree sample (init)\n");

        let options = kernel::miscdevice::MiscDeviceOptions {
            name: kernel::c_str!("rust-misc-device"),
        };

        try_pin_init!(Self {
            _miscdev <- kernel::miscdevice::MiscDeviceRegistration::register(options),
        })
    }
}

#[pinned_drop]
impl PinnedDrop for RustOutOfTree {
    fn drop(self: Pin<&mut Self>) {
        pr_info!("Rust out-of-tree sample (exit)\n");
    }
}

#[pin_data]
struct RustOutOfTreeDevice {
    dev: kernel::types::ARef<kernel::device::Device>,
}

#[vtable]
impl kernel::miscdevice::MiscDevice for RustOutOfTreeDevice {
    type Ptr = Pin<KBox<Self>>;

    fn open(
        file: &kernel::fs::File,
        misc: &kernel::miscdevice::MiscDeviceRegistration<Self>,
    ) -> Result<Pin<KBox<Self>>> {
        let dev = kernel::types::ARef::from(misc.device());

        dev_info!(
            dev,
            "Opening Rust Misc Device Sample (uid = {})\n",
            file.cred().euid().into_uid_in_current_ns()
        );

        KBox::try_pin_init(
            try_pin_init! {
                RustOutOfTreeDevice {
                    dev: dev,
                }
            },
            GFP_KERNEL,
        )
    }
}
```

</details>

Now we should be able to see a log when we try and open our device.

```console
$ make KDIR=../linux-6.14 LLVM=1
$ insmod ./rust_out_of_tree.ko
[ 3918.696311] rust_out_of_tree: Rust out-of-tree sample (init)
$ cat /dev/rust-misc-device
cat: /dev/rust-misc-device: Invalid argument
[ 3990.836103] misc rust-misc-device: Opening Rust Misc Device Sample (uid = 0)
```

Great, we can see that the device has been opened by `cat`. As of Linux 6.14
there's no support for Read operations - only `ioctl` operations - so `cat` gets
an error from the Kernel. That's expected.

## Task 10 - implement `ioctl`

You've got the hang of this now, so as a bonus exercise, why not implement
`ioctl`. See
<https://rust.docs.kernel.org/kernel/miscdevice/trait.MiscDevice.html> for
details.

You'll need some IOCTL numbers to use. Try creating a "Hello" `ioctl`, with no
argument (no data read and no data written):

```rust ignore
const RUST_MISC_DEV_HELLO: u32 = _IO('|' as u32, 0x80);
```

I chose `|` as the `ioctl` type for Miscellaneous Devices, because that's what
is in the [example code]. If you need help with this step, that's a great place
to look.

[example code]: https://github.com/torvalds/linux/blob/v6.14/samples/rust/rust_misc_device.rs

To send an ioctl to your device, you can use this Rust program. You'll need to
put it in a package (`cargo new --bin openfile`) and add the `nix` crate (`cargo
add nix`).

```rust ignore
use std::os::fd::AsRawFd;

const HELLO: u8 = 0x80;
nix::ioctl_none!(hello_ioctl, '|', HELLO);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::File::open("/dev/rust-misc-device")?;
    let fd = f.as_raw_fd();
    let result = unsafe { hello_ioctl(fd) };
    println!("ioctl returned {:?}", result);
    Ok(())
}
```

Or you could try and write the equivalent in Rust (you'll probably need the
`nix` crate and the `libc` crate).

<details>
<summary>Here's an example `ioctl` method if you need one</summary>

```rust ignore
fn ioctl(
    me: Pin<&RustOutOfTreeDevice>,
    _file: &kernel::fs::File,
    cmd: u32,
    arg: usize,
) -> Result<isize> {
    dev_info!(me.dev, "IOCTLing Rust Out Of Tree Device\n");

    let size = kernel::ioctl::_IOC_SIZE(cmd);

    match cmd {
        RUST_MISC_DEV_HELLO => {
            dev_info!(me.dev, "-> hello received (size {}, arg {})\n", size, arg);
            Ok(100)
        }
        _ => {
            dev_err!(me.dev, "-> IOCTL not recognised: {}\n", cmd);
            Err(ENOTTY)
        }
    }
}
```

</details>

Here's what your output might look like if we run that example:

```console
$ make KDIR=../linux-6.14 LLVM=1
$ insmod ./rust_out_of_tree.ko
[12147.696311] rust_out_of_tree: Rust out-of-tree sample (init)
$ cd ../openfile
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/openfile`
ioctl returned Ok(100)
$ dmesg
[12150.540427] misc rust-misc-device: Opening Rust Misc Device Sample (uid = 0)
[12150.541318] misc rust-misc-device: IOCTLing Rust Misc Device Sample
[12150.541606] misc rust-misc-device: -> hello received (size 0, arg 0)
```

## Task 11 - Keep going!

OK! If you still want more, try implementing 'read' and 'write' `ioctl`s, so you
can communicate with your driver. Or look at the Kernel mailing list for the
patches that will let you do ordinary `read` and `write` syscalls on your
device, rather than just `ioctls`. Happy kernel hacking in Rust!
