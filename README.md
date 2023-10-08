# Health Analyzer
## Intro:
This project is a simple grpc server streaming service, where the client sets a monitoring duration and a sampling rate for the service to report back health data (cpu utilization, memory utilization, disk utilizatio, ... etc. ).
This is done over https. A dummy 3 tier PKI cert chain was created with the steps listed below to support this, the steps are aggregated in the `gen.sh` and `gen.ps1` scripts.



To create a 3-tier certificate chain using OpenSSL and configuration files, you can follow the steps below:

1. Create Root Certificate Authority and Private Key:
```
openssl req -new -x509 -newkey rsa:2048 -keyout .\root\root.key -out .\root\root.pem -config .\root\root.cnf -outform PEM -nodes
```

2. Create Intermediate Certificate Authority CSR file and Private Key:
```
openssl req -new -newkey rsa:2048 -keyout .\ca\ca.key -out .\ca\ca.csr -config .\ca\ca.cnf -nodes
```

3. Sign Intermediate Certificate Authority CSR file with Root Certificate Authority Private Key:
```
openssl x509 -req -in .\ca\ca.csr -CA .\root\root.pem -CAkey .\root\root.key -CAcreateserial -out .\ca\ca.pem -days 365 -sha256 -extfile .\ca\ca.cnf -extensions cert_ext
```

4. Create Client/Server side Certificate CSR file and Private Key:
```
openssl req -new -newkey rsa:2048 -keyout .\client-leaf\client-leaf.key -out .\client-leaf\client-leaf.csr -config .\client-leaf\leaf.cnf -nodes

openssl req -new -newkey rsa:2048 -keyout .\server-leaf\server-leaf.key -out .\server-leaf\server-leaf.csr -config .\server-leaf\leaf.cnf -nodes
```

5. Sign Client/Server side Certificate PEM file with Intermediate Certificate Authority Private Key:
```
openssl x509 -req -in .\client-leaf\client-leaf.csr -CA .\ca\ca.pem -CAkey .\ca\ca.key -CAcreateserial -out .\client-leaf\client-leaf.pem -days 365 -sha256 -extfile .\client-leaf\leaf.cnf -extensions req_ext

openssl x509 -req -in .\server-leaf\server-leaf.csr -CA .\ca\ca.pem -CAkey .\ca\ca.key -CAcreateserial -out .\server-leaf\server-leaf.pem -days 365 -sha256 -extfile .\server-leaf\leaf.cnf -extensions req_ext
```

6. Create chain.pem file
```
cat .\ca\ca.pem > chain.pem && cat .\root\root.pem >> chain.pem
```

Sample usage:
```
PS C:\Users\zeyadhabib\source\repos\rust\health_analyzer> .\target\release\status-client.exe -a "https://[::1]" -p 50051 -d "zeyad.server.com" -s 5 -D 2  
Connecting to https://[::1]:50051
Response: Response { metadata: MetadataMap { headers: {"content-type": "application/grpc", "date": "Sun, 08 Oct 2023 14:44:09 GMT", "grpc-status": "0"} }, message: SpecsResponse { total_ram: 34185707520, total_disk: 5023715024896, total_cpu: [3302.0, 3302.0, 3302.0, 3302.0, 1200.0, 3302.0, 3302.0, 3302.0] }, extensions: Extensions }
Response: StatusResponse { used_ram: 18561753088, used_disk: 1146137624576, used_cpu: [68.9569091796875, 24.65087890625, 35.854583740234375, 35.80949401855469, 95.58187866210938, 28.597557067871094, 30.04754638671875, 33.998939514160156] }
Response: StatusResponse { used_ram: 18562985984, used_disk: 1146137624576, used_cpu: [31.1510009765625, 27.69245147705078, 27.204879760742188, 29.31311798095703, 24.980018615722656, 46.53385543823242, 66.60494995117188, 24.994781494140625] }
Response: StatusResponse { used_ram: 18563117056, used_disk: 1146137640960, used_cpu: [54.98541259765625, 32.554710388183594, 37.63869094848633, 29.6162109375, 30.841064453125, 40.3007926940918, 51.282470703125, 27.00433349609375] }
Response: StatusResponse { used_ram: 18562809856, used_disk: 1146137640960, used_cpu: [51.67756271362305, 33.048255920410156, 68.3060073852539, 37.34992599487305, 48.67124938964844, 40.486759185791016, 63.45112609863281, 38.16585159301758] }
Response: StatusResponse { used_ram: 18560913408, used_disk: 1146137640960, used_cpu: [70.66140747070313, 46.66929244995117, 75.17117309570313, 49.27429962158203, 51.91347885131836, 52.92964172363281, 57.674476623535156, 48.24125289916992] }
Response: StatusResponse { used_ram: 18562285568, used_disk: 1146137640960, used_cpu: [69.58261108398438, 55.16838073730469, 62.539798736572266, 55.74290084838867, 65.27752685546875, 46.501399993896484, 55.819984436035156, 53.67470169067383] }
Response: StatusResponse { used_ram: 18564968448, used_disk: 1146137640960, used_cpu: [65.53102111816406, 49.57522964477539, 66.52780151367188, 56.178287506103516, 59.99479293823242, 53.07948303222656, 68.37957763671875, 60.65159606933594] }
Response: StatusResponse { used_ram: 18565718016, used_disk: 1146137640960, used_cpu: [35.20941162109375, 43.87556457519531, 43.82141876220703, 32.76983642578125, 36.91781234741211, 29.204788208007813, 40.904632568359375, 53.8170166015625] }
Response: StatusResponse { used_ram: 18565324800, used_disk: 1146137640960, used_cpu: [75.07536315917969, 25.17053985595703, 33.51393127441406, 32.153961181640625, 36.226806640625, 31.94336700439453, 30.06109619140625, 31.846961975097656] }
Response: StatusResponse { used_ram: 18532093952, used_disk: 1146137640960, used_cpu: [27.68480682373047, 42.97907257080078, 21.860267639160156, 50.149444580078125, 36.5574951171875, 24.300064086914063, 38.11235809326172, 20.129776000976563] }
```