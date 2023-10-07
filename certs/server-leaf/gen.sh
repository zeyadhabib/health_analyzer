#! /bin/bash

openssl req -new -config .\leaf.cnf -nodes -keyout key.pem -out leaf.csr
openssl x509 -req -in .\leaf.csr -days 730 -CA ..\ca\ca.pem -CAkey ..\ca\key.pem -extensions req_ext -extfile .\leaf.cnf -out leaf.pem