# Command Recall

A CLI tool connected to GPT-3 to help find the right terminal command 

## Install
to install the cli:
`cargo install --git https://github.com/CamilleMo/command_recall`

## Configure token
To use this project, an openai token is needed.  
Get one at `https://platform.openai.com/account/api-keys`  

then configure the cli with your token:  
`command_recall configure --token <token>`  
or if you don't want to input your token in the terminal:  
`command_recall configure`  
It will create the config file without token  
then input your token manually in the config file.  

## Usage

Optionally create an alias: `alias cr='command_recall ask --task'`


```bash

command_recall ask --task "find my raspberry on my LAN"
┌────────┬──────────────┬─────────────────────────────────────────────┐
│ number │ command      │ description                                 │
├────────┼──────────────┼─────────────────────────────────────────────┤
│ 1.     │ nmap -sn     │ Network Mapper - Ping Scan                  │
│ 2.     │ arp-scan     │ ARP Scanner                                 │
│ 3.     │ avahi-browse │ Browse for mDNS/DNS-SD services             │
│ 4.     │ nbtscan      │ NetBIOS Name Service Scanner                │
│ 5.     │ nmap -sP     │ Network Mapper - Ping Scan                  │
│ 6.     │ nmap -sL     │ Network Mapper - List Scan                  │
│ 7.     │ nmap -sU     │ Network Mapper - UDP Scan                   │
│ 8.     │ nmap -sV     │ Network Mapper - Version Detection          │
│ 9.     │ nmap -O      │ Network Mapper - Operating System Detection │
│ 10.    │ nmap -A      │ Network Mapper - Aggressive Scan            │
└────────┴──────────────┴─────────────────────────────────────────────┘

command_recall ask --task "mount my hard drive"
┌────────┬────────────┬──────────────────────────────────────────┐
│ number │ command    │ description                              │
├────────┼────────────┼──────────────────────────────────────────┤
│ 1.     │ mount      │ Mount a filesystem                       │
│ 2.     │ fdisk      │ Manipulate disk partition table          │
│ 3.     │ blkid      │ Locate/print block device attributes     │
│ 4.     │ lsblk      │ List information about block devices     │
│ 5.     │ mkfs       │ Create a filesystem                      │
│ 6.     │ parted     │ Partition table manipulator              │
│ 7.     │ mountpoint │ Determine if a directory is a mountpoint │
│ 8.     │ df         │ Report file system disk space usage      │
│ 9.     │ umount     │ Unmount file systems                     │
│ 10.    │ fsck       │ Check and repair a Linux filesystem      │
└────────┴────────────┴──────────────────────────────────────────┘

command_recall ask --task "mount my hard drive using mount"
┌────────┬─────────────────────────────────────────────────────┬──────────────────────────┐
│ number │ command                                             │ description              │
├────────┼─────────────────────────────────────────────────────┼──────────────────────────┤
│ 1.     │ mount -t auto /dev/sda1 /mnt/mydrive                │ Mount a filesystem       │
│ 2.     │ mount -o remount,rw /dev/sda1 /mnt/mydrive          │ Remount a filesystem     │
│ 3.     │ mount -o loop /mnt/mydrive/myimage.iso /mnt/mydrive │ Mount a loopback device  │
│ 4.     │ mount -t ntfs-3g /dev/sda1 /mnt/mydrive             │ Mount an NTFS filesystem │
│ 5.     │ mount -t ext4 /dev/sda1 /mnt/mydrive                │ Mount an ext4 filesystem │
│ 6.     │ mount -t vfat /dev/sda1 /mnt/mydrive                │ Mount a VFAT filesystem  │
│ 7.     │ mount -t cifs //server/share /mnt/mydrive           │ Mount a CIFS share       │
│ 8.     │ mount -t smbfs //server/share /mnt/mydrive          │ Mount an SMBFS share     │
│ 9.     │ mount -t nfs server:/export /mnt/mydrive            │ Mount an NFS share       │
│ 10.    │ mount -t tmpfs -o size=1024m /mnt/mydrive           │ Mount a tmpfs filesystem │
└────────┴─────────────────────────────────────────────────────┴──────────────────────────┘

```

## Notes

 - Even if the temperature is set to 0 the model is not determinist. Response can change.
 - Timeouts happen frequently.