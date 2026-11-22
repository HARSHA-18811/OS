#!/bin/bash
set -e

CHROOT="/var/tmp/micro-os-build/chroot"
echo "===> Applying Windows 11-style Wi-Fi Quick Settings & Tray Applet to $CHROOT..."

# 1. Install micro-wifi script
cp -f "/mnt/d/mydata/Documents/3rd sem/os/OS/micro-os/scripts/micro-wifi.py" "$CHROOT/usr/local/bin/micro-wifi"
chmod 755 "$CHROOT/usr/local/bin/micro-wifi"
ln -sf /usr/local/bin/micro-wifi "$CHROOT/usr/bin/wifi"

# 2. Create Application desktop entry
cat << 'EOF' > "$CHROOT/usr/share/applications/micro-wifi.desktop"
[Desktop Entry]
Version=1.0
Type=Application
Name=Wi-Fi Networks
Comment=Connect to Wi-Fi networks and view available hotspots
Exec=/usr/local/bin/micro-wifi --show
Icon=network-wireless
Categories=Network;Settings;
Terminal=false
StartupNotify=false
EOF
chmod 644 "$CHROOT/usr/share/applications/micro-wifi.desktop"

# 3. Create Autostart entry
mkdir -p "$CHROOT/etc/xdg/autostart" "$CHROOT/home/user/.config/autostart" "$CHROOT/etc/skel/.config/autostart"
cat << 'EOF' > "$CHROOT/etc/xdg/autostart/micro-wifi.desktop"
[Desktop Entry]
Version=1.0
Type=Application
Name=MicroOS Wi-Fi Tray
Comment=Wi-Fi Status and Quick Settings in System Tray
Exec=/usr/local/bin/micro-wifi
Icon=network-wireless
Terminal=false
StartupNotify=false
X-GNOME-Autostart-enabled=true
EOF
chmod 644 "$CHROOT/etc/xdg/autostart/micro-wifi.desktop"
cp -f "$CHROOT/etc/xdg/autostart/micro-wifi.desktop" "$CHROOT/home/user/.config/autostart/micro-wifi.desktop"
cp -f "$CHROOT/etc/xdg/autostart/micro-wifi.desktop" "$CHROOT/etc/skel/.config/autostart/micro-wifi.desktop"

# 4. Configure XFCE Panel launcher button for Wi-Fi (placed right in the corner next to clock)
mkdir -p "$CHROOT/home/user/.config/xfce4/panel/launcher-6" "$CHROOT/etc/skel/.config/xfce4/panel/launcher-6"
cat << 'EOF' > "$CHROOT/home/user/.config/xfce4/panel/launcher-6/micro-wifi.desktop"
[Desktop Entry]
Version=1.0
Type=Application
Name=Wi-Fi Networks
Comment=Connect to Wi-Fi networks and view available hotspots
Exec=/usr/local/bin/micro-wifi --show
Icon=network-wireless
Path=/home/user
Terminal=false
StartupNotify=false
EOF
chmod 755 "$CHROOT/home/user/.config/xfce4/panel/launcher-6/micro-wifi.desktop"
cp -f "$CHROOT/home/user/.config/xfce4/panel/launcher-6/micro-wifi.desktop" "$CHROOT/etc/skel/.config/xfce4/panel/launcher-6/micro-wifi.desktop"

# 5. Update xfce4-panel.xml to include launcher-6 right next to clock
mkdir -p "$CHROOT/home/user/.config/xfce4/xfconf/xfce-perchannel-xml" "$CHROOT/etc/skel/.config/xfce4/xfconf/xfce-perchannel-xml"
cat << 'EOF' > "$CHROOT/home/user/.config/xfce4/xfconf/xfce-perchannel-xml/xfce4-panel.xml"
<?xml version="1.0" encoding="UTF-8"?>
<channel name="xfce4-panel" version="1.0">
  <property name="configver" type="int" value="2"/>
  <property name="panels" type="array">
    <value type="int" value="1"/>
    <property name="dark-mode" type="bool" value="true"/>
    <property name="panel-1" type="empty">
      <property name="position" type="string" value="p=8;x=0;y=0"/>
      <property name="length" type="uint" value="100"/>
      <property name="position-locked" type="bool" value="true"/>
      <property name="icon-size" type="uint" value="20"/>
      <property name="size" type="uint" value="36"/>
      <property name="plugin-ids" type="array">
        <value type="int" value="1"/>
        <value type="int" value="2"/>
        <value type="int" value="3"/>
        <value type="int" value="4"/>
        <value type="int" value="6"/>
        <value type="int" value="5"/>
      </property>
    </property>
  </property>
  <property name="plugins" type="empty">
    <property name="plugin-1" type="string" value="applicationsmenu">
      <property name="button-title" type="string" value="Start"/>
      <property name="show-button-title" type="bool" value="true"/>
      <property name="button-icon" type="string" value="distributor-logo"/>
    </property>
    <property name="plugin-2" type="string" value="tasklist">
      <property name="grouping" type="uint" value="1"/>
      <property name="show-labels" type="bool" value="true"/>
      <property name="flat-buttons" type="bool" value="false"/>
    </property>
    <property name="plugin-3" type="string" value="separator">
      <property name="expand" type="bool" value="true"/>
      <property name="style" type="uint" value="0"/>
    </property>
    <property name="plugin-4" type="string" value="systray">
      <property name="square-icons" type="bool" value="true"/>
    </property>
    <property name="plugin-6" type="string" value="launcher">
      <property name="items" type="array">
        <value type="string" value="micro-wifi.desktop"/>
      </property>
    </property>
    <property name="plugin-5" type="string" value="clock">
      <property name="digital-time-format" type="string" value="%l:%M %p"/>
    </property>
  </property>
</channel>
EOF
cp -f "$CHROOT/home/user/.config/xfce4/xfconf/xfce-perchannel-xml/xfce4-panel.xml" "$CHROOT/etc/skel/.config/xfce4/xfconf/xfce-perchannel-xml/xfce4-panel.xml"

# 6. Update Desktop shortcut
mkdir -p "$CHROOT/home/user/Desktop" "$CHROOT/etc/skel/Desktop"
cat << 'EOF' > "$CHROOT/home/user/Desktop/network.desktop"
[Desktop Entry]
Version=1.0
Type=Application
Name=Wi-Fi Settings
Comment=Connect to Wi-Fi networks and view available hotspots
Exec=/usr/local/bin/micro-wifi --window
Icon=network-wireless
Path=/home/user
Terminal=false
StartupNotify=true
Categories=Network;Settings;
EOF
chmod 755 "$CHROOT/home/user/Desktop/network.desktop"
cp -f "$CHROOT/home/user/Desktop/network.desktop" "$CHROOT/etc/skel/Desktop/network.desktop"

# 7. Pre-seed default scanned Wi-Fi list with real hostel & campus SSIDs
cat << 'EOF' > "$CHROOT/home/user/.scanned_wifis"
SSID 1 : MOJO-JAN-C1-F8
SSID 2 : MOJO-JAN-C1-F9
SSID 3 : OPPO F29 Pro 5G 232d
SSID 4 : MOJO-JAN-C1-F6
SSID 5 : MOJO-JAN-C1-F7
SSID 6 : Jain_Student
SSID 7 : Jain_Staff
EOF
cp -f "$CHROOT/home/user/.scanned_wifis" "$CHROOT/etc/skel/.scanned_wifis"

# Set permissions
chown -R 1000:1000 "$CHROOT/home/user"

echo "===> Successfully updated Wi-Fi subsystem and Windows-style UI in chroot!"
