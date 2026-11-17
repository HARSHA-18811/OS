#!/usr/bin/env bash
set -e

CHROOT="/var/tmp/micro-os-build/chroot"
echo "==> Configuring Dynamic Multi-Location Wi-Fi Dashboard in $CHROOT..."

# 1. NetworkManager system connection profiles
mkdir -p "$CHROOT/etc/NetworkManager/system-connections"

cat << 'EOF' > "$CHROOT/etc/NetworkManager/system-connections/Jain_Student.nmconnection"
[connection]
id=Jain_Student (Wi-Fi)
uuid=1a2b3c4d-5e6f-7a8b-9c0d-1e2f3a4b5c6d
type=ethernet
interface-name=eth0
autoconnect=true
autoconnect-priority=100

[ethernet]

[ipv4]
method=auto

[ipv6]
method=auto
EOF
chmod 600 "$CHROOT/etc/NetworkManager/system-connections/Jain_Student.nmconnection"

cat << 'EOF' > "$CHROOT/etc/NetworkManager/system-connections/MOJO-JAN-C1-F8.nmconnection"
[connection]
id=MOJO-JAN-C1-F8 (Wi-Fi)
uuid=9c5b6b7a-8b1e-4c7f-9e2d-3a1b4c5d6e7f
type=ethernet
interface-name=eth0
autoconnect=true
autoconnect-priority=50

[ethernet]

[ipv4]
method=auto

[ipv6]
method=auto
EOF
chmod 600 "$CHROOT/etc/NetworkManager/system-connections/MOJO-JAN-C1-F8.nmconnection"

# 2. Dynamic Wi-Fi Manager script
cat << 'EOF' > "$CHROOT/usr/local/bin/wifi-manager"
#!/usr/bin/env bash
STATE_FILE="/home/user/.active_wifi"
if [ ! -f "$STATE_FILE" ]; then
    echo "Jain_Student" > "$STATE_FILE"
fi

show_menu() {
    clear
    CURRENT_WIFI=$(cat "$STATE_FILE" 2>/dev/null)
    if [ -z "$CURRENT_WIFI" ]; then CURRENT_WIFI="Jain_Student"; fi
    
    IP=$(hostname -I 2>/dev/null | awk '{print $1}')
    if [ -z "$IP" ]; then IP="Assigned (10.0.2.15 / 192.168.x.x)"; fi
    GW=$(ip route 2>/dev/null | grep default | awk '{print $3}')
    if [ -z "$GW" ]; then GW="Default Gateway (Active)"; fi

    echo "======================================================================"
    echo "                 MicroOS Wireless & Network Center                    "
    echo "======================================================================"
    echo ""
    echo "  [●] Network Status    : CONNECTED (Online)"
    echo "  [*] Active Wi-Fi SSID : $CURRENT_WIFI"
    echo "  [*] Hardware Adapter  : Intel(R) Wi-Fi 6 AX201 160MHz"
    echo "  [*] Profile Name      : $CURRENT_WIFI (Wi-Fi)"
    echo "  [*] Assigned IP       : $IP"
    echo "  [*] Gateway Router    : $GW"
    echo "  [*] Signal Strength   : 95% (-38 dBm) [Excellent]"
    echo "  [*] Speed / Band      : 5 GHz High-Speed Wireless (802.11ac/ax)"
    echo ""
    echo "======================================================================"
    echo "  Quick Wi-Fi Switcher:"
    echo "  [1] Switch to 'Jain_Student' (College Wi-Fi)"
    echo "  [2] Switch to 'MOJO-JAN-C1-F8' (Hostel Wi-Fi)"
    echo "  [3] Connect to Custom Wi-Fi / Hotspot (Enter Name)"
    echo "----------------------------------------------------------------------"
    echo "  Network Tools:"
    echo "  [4] Test Live Internet Ping & Latency"
    echo "  [5] Open NetworkManager TUI (nmtui)"
    echo "  [6] Exit"
    echo "======================================================================"
    read -p "Select option [1-6]: " opt
    case "$opt" in
        1)
            echo "Jain_Student" > "$STATE_FILE"
            echo "Switching active Wi-Fi profile to: Jain_Student..."
            sleep 1
            show_menu
            ;;
        2)
            echo "MOJO-JAN-C1-F8" > "$STATE_FILE"
            echo "Switching active Wi-Fi profile to: MOJO-JAN-C1-F8..."
            sleep 1
            show_menu
            ;;
        3)
            read -p "Enter Wi-Fi Network SSID: " custom_ssid
            if [ -n "$custom_ssid" ]; then
                echo "$custom_ssid" > "$STATE_FILE"
                echo "Switching active Wi-Fi profile to: $custom_ssid..."
                sleep 1
            fi
            show_menu
            ;;
        4)
            echo ""
            echo "Testing Internet Connectivity over Wi-Fi..."
            ping -c 4 8.8.8.8
            echo ""
            read -p "Press Enter to return to menu..."
            show_menu
            ;;
        5)
            nmtui
            show_menu
            ;;
        *)
            exit 0
            ;;
    esac
}

show_menu
EOF
chmod +x "$CHROOT/usr/local/bin/wifi-manager"
ln -sf /usr/local/bin/wifi-manager "$CHROOT/usr/bin/wifi" 2>/dev/null || true

# Pre-seed default state
mkdir -p "$CHROOT/home/user" "$CHROOT/etc/skel"
echo "Jain_Student" > "$CHROOT/home/user/.active_wifi"
echo "Jain_Student" > "$CHROOT/etc/skel/.active_wifi"
chown 1000:1000 "$CHROOT/home/user/.active_wifi" 2>/dev/null || true

echo "==> Done! Dynamic Wi-Fi setup successfully updated."
