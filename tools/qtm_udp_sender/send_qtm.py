"""
send_qtm.py

Python3 script to send QTM-like CaptureStart/CaptureStop UDP packets.
Usage:
    python send_qtm.py --mode start --name mycapture --database myfolder --packetid 1 --host 127.0.0.1 --port 8989
"""
import argparse
import socket
import os

parser = argparse.ArgumentParser()
parser.add_argument("--mode", choices=["start","stop"], required=True)
parser.add_argument("--name", default="test")
parser.add_argument("--database", default="")
parser.add_argument("--packetid", type=int, default=1)
parser.add_argument("--host", default="127.0.0.1")
parser.add_argument("--port", type=int, default=8989)
parser.add_argument("--hostname", default=os.environ.get("COMPUTERNAME") or os.environ.get("HOSTNAME") or "")
parser.add_argument("--pid", type=int, default=99999)
parser.add_argument("--result", choices=["SUCCESS","FAIL","CANCEL"], default="SUCCESS")

args = parser.parse_args()

if args.mode == "start":
    xml = f'''<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<CaptureStart>
<Name VALUE="{args.name}"/>
<DatabasePath VALUE="{args.database}"/>
<Delay VALUE="0"/>
<PacketID VALUE="{args.packetid}"/>
<HostName VALUE="{args.hostname}"/>
<ProcessID VALUE="{args.pid}"/>
<Notes VALUE=""/>
<Description VALUE=""/>
<Timecode VALUE=""/>
</CaptureStart>
'''
else:
    xml = f'''<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<CaptureStop RESULT="{args.result}">
<Name VALUE="{args.name}"/>
<DatabasePath VALUE="{args.database}"/>
<Delay VALUE="0"/>
<PacketID VALUE="{args.packetid}"/>
<HostName VALUE="{args.hostname}"/>
<ProcessID VALUE="{args.pid}"/>
</CaptureStop>
'''

s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
try:
    s.sendto(xml.encode('utf-8'), (args.host, args.port))
    print(f"Sent {args.mode} packet to {args.host}:{args.port} (PacketID={args.packetid})")
finally:
    s.close()
