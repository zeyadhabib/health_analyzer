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
