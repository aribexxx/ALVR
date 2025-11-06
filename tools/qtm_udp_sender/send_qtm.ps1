<#
send_qtm.ps1

Sends a QTM-like CaptureStart or CaptureStop UDP packet.
#>
param(
    [Parameter(Mandatory=$true)]
    [ValidateSet('start','stop')]
    [string]$Mode,

    [string]$Name = "test",
    [string]$DatabasePath = "",
    [int]$PacketID = 1,

    [string]$TargetHost = "127.0.0.1",
    [int]$Port = 8989,

    [string]$HostName = $env:COMPUTERNAME,
    [int]$ProcessID = 99999,

    [ValidateSet('SUCCESS','FAIL','CANCEL')]
    [string]$Result = "SUCCESS"
)

# Build XML payload
if ($Mode -eq 'start') {
    $xml = @"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<CaptureStart>
<Name VALUE="$Name"/>
<DatabasePath VALUE="$DatabasePath"/>
<Delay VALUE="0"/>
<PacketID VALUE="$PacketID"/>
<HostName VALUE="$HostName"/>
<ProcessID VALUE="$ProcessID"/>
<Notes VALUE=""/>
<Description VALUE=""/>
<Timecode VALUE=""/>
</CaptureStart>
"@
} else {
    $xml = @"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<CaptureStop RESULT="$Result">
<Name VALUE="$Name"/>
<DatabasePath VALUE="$DatabasePath"/>
<Delay VALUE="0"/>
<PacketID VALUE="$PacketID"/>
<HostName VALUE="$HostName"/>
<ProcessID VALUE="$ProcessID"/>
</CaptureStop>
"@
}

# Convert to bytes and send
$bytes = [System.Text.Encoding]::UTF8.GetBytes($xml)
$udp = New-Object System.Net.Sockets.UdpClient
try {
    $udp.Connect($TargetHost, $Port)
    $udp.Send($bytes, $bytes.Length) | Out-Null
    Write-Host "Sent $Mode packet to $TargetHost`:$Port (PacketID=$PacketID)" -ForegroundColor Green
} catch {
    Write-Error "Failed to send UDP packet: $_"
} finally {
    $udp.Close()
}
