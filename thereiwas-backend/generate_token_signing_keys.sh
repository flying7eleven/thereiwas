#!/bin/bash
openssl genpkey -algorithm Ed25519 -out ed.private.key
openssl pkcs8 -topk8 -nocrypt -in ed.private.key -out ed.private.pem
openssl pkey -in ed.private.pem -pubout -out ed.public.pem
