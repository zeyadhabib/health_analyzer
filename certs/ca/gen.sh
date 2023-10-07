#! /bin/bash

openssl req -x509 -config .\ca.cnf -nodes -keyout key.pem -out ca.pem -outform PEM