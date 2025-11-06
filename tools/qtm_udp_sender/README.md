QTM UDP Sender

This small tool contains two scripts to simulate QTM CaptureStart / CaptureStop broadcast UDP packets.

Files:
- `send_qtm.ps1` - PowerShell script (Windows). Recommended for quick testing.
- `send_qtm.py` - Python3 script alternative.

Default target: localhost:8989 (QTM default). You can change host/port and packet fields via parameters.

PowerShell example (send a CaptureStart):

```powershell
# send start to local machine
.\send_qtm.ps1 -Mode start -Name mycapture -DatabasePath myfolder -PacketID 123 -TargetHost 127.0.0.1 -Port 8989

# send stop
.\send_qtm.ps1 -Mode stop -Name mycapture -DatabasePath myfolder -PacketID 124 -TargetHost 127.0.0.1 -Port 8989 -Result SUCCESS
```

Python example (send a CaptureStart):

```powershell
python .\send_qtm.py --mode start --name mycapture --database myfolder --packetid 1 --host 127.0.0.1 --port 8989
```

Use these scripts while the ALVR server is running and streaming to validate start/stop behavior.
