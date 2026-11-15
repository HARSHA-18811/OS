Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public class WinAct {
    [DllImport("user32.dll")]
    public static extern bool ShowWindow(IntPtr hWnd, int nCmdShow);
    [DllImport("user32.dll")]
    public static extern bool SetForegroundWindow(IntPtr hWnd);
    [DllImport("user32.dll")]
    public static extern bool EnumWindows(EnumWindowsProc lpEnumFunc, IntPtr lParam);
    public delegate bool EnumWindowsProc(IntPtr hWnd, IntPtr lParam);
    [DllImport("user32.dll", CharSet = CharSet.Auto)]
    public static extern int GetWindowText(IntPtr hWnd, System.Text.StringBuilder lpString, int nMaxCount);
}
"@

[WinAct]::EnumWindows({
    param($hWnd, $lParam)
    $sb = New-Object System.Text.StringBuilder 256
    [WinAct]::GetWindowText($hWnd, $sb, 256) | Out-Null
    $title = $sb.ToString()
    if ($title -like "*MicroOS*") {
        Write-Host "Found VM Window: $title"
        [WinAct]::ShowWindow($hWnd, 9) | Out-Null # SW_RESTORE
        [WinAct]::ShowWindow($hWnd, 5) | Out-Null # SW_SHOW
        [WinAct]::SetForegroundWindow($hWnd) | Out-Null
    }
    return $true
}, [IntPtr]::Zero) | Out-Null
