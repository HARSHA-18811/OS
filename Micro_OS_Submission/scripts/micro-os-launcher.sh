#!/bin/bash
# Micro OS Interactive Terminal Launcher
clear
cat << "EOF"
================================================================================
       __  __ _                    ____   _____ 
      |  \/  (_)                  / __ \ / ____|
      | \  / |_  ___ _ __ ___    | |  | | (___  
      | |\/| | |/ __| '__/ _ \   | |  | |\___ \ 
      | |  | | | (__| | | (_) |  | |__| |____) |
      |_|  |_|_|\___|_|  \___/    \____/|_____/ 
                                                
   Debian 12 Bookworm (Minimal) - Laptop Compatible - Live ISO
================================================================================
  Hardware Architecture : x86_64 (UEFI / Legacy BIOS Hybrid)
  Kernel                : Linux 6.1+ (WiFi, LAN & Laptop Drivers Loaded)
  Package Manager       : APT & APTITUDE (Debian Mirrors Configured)
  App Container Store   : SNAPSTORE (snapd service active)
  Research Lab          : Producer-Consumer Performance Lab (pre-installed)
================================================================================
EOF

while true; do
    echo ""
    echo "  [1] Producer-Consumer Performance Research Lab (Academic Feature)"
    echo "  [2] Open Shell Terminal (bash)"
    echo "  [3] Network & WiFi Configuration (NetworkManager / nmtui)"
    echo "  [4] Package Manager: APT / Aptitude Demonstration"
    echo "  [5] App Store: Snapstore / snapd Demonstration"
    echo "  [6] Boot Performance Benchmark (systemd-analyze)"
    echo "  [7] System Hardware & Laptop Specs (lshw, lspci, ip)"
    echo "  [8] Power Off / Reboot System"
    echo "================================================================================"
    read -p "Select option (1-8): " choice

    case "$choice" in
        1)
            /usr/local/bin/pclab
            ;;
        2)
            echo "Entering interactive bash shell. Type 'exit' to return to launcher."
            /bin/bash
            ;;
        3)
            echo "Launching NetworkManager TUI..."
            if command -v nmtui &> /dev/null; then
                nmtui
            else
                nmcli device status
            fi
            ;;
        4)
            echo "=== Package Manager: APT and Aptitude ==="
            echo "1. Checking apt version: $(apt --version)"
            echo "2. Checking aptitude version: $(aptitude --version | head -n 1)"
            echo "Try running: sudo apt update && sudo apt install <package>"
            echo "Try running: sudo aptitude"
            read -p "Press Enter to continue..."
            ;;
        5)
            echo "=== App Store: Snap / Snapstore ==="
            echo "Snap daemon version: $(snap version | head -n 2)"
            echo "Try searching packages: snap find <query>"
            echo "Try installing packages: sudo snap install <package>"
            read -p "Press Enter to continue..."
            ;;
        6)
            echo "=== Boot Performance Analysis ==="
            systemd-analyze
            echo ""
            echo "Top startup services breakdown:"
            systemd-analyze blame | head -n 10
            read -p "Press Enter to continue..."
            ;;
        7)
            echo "=== Hardware & System Information ==="
            uname -a
            echo "--- Network Interfaces ---"
            ip -brief addr
            echo "--- PCI Devices ---"
            lspci | head -n 10
            read -p "Press Enter to continue..."
            ;;
        8)
            echo "Select: [1] Reboot, [2] Poweroff, [3] Cancel"
            read -p "Option: " p_choice
            case "$p_choice" in
                1) sudo reboot ;;
                2) sudo poweroff ;;
                *) echo "Canceled." ;;
            esac
            ;;
        *)
            echo "Invalid option. Please choose 1-8."
            ;;
    esac
done
