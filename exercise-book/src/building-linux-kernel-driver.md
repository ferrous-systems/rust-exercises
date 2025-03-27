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
scroll down to *Rust support*. Presss `Y` to enable Rust support. Then use the
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

## Task 8 - Make a (somewhat) useful kernel module

Your task is to write a kernel module that has a character device -
`/dev/maths`. When we write numbers (as decimals in ASCII) to the device, the
kernel module is going to keep a running total. When we read from the device,
it's going to give us the total. If we send an invalid number it's going to zero
the total.

To do this, we'll need to use the character device API.

```console
$ echo 100 > /dev/maths
$ echo 200 > /dev/maths
$ cat /dev/maths
300 
$ echo 1 > /dev/maths
$ cat /dev/maths
301
$ echo foo > /dev/maths
$ cat /dev/maths
0
```

TODO: is this even possible?
