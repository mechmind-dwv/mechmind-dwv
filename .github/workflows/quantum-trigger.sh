#!/bin/bash
# Script de activación manual
curl -X POST \
  -H "Authorization: Bearer ${{ secrets.OMEGA_KEY }}" \
  -H "Content-Type: application/json" \
  -d '{"event_type":"quantum_scan"}' \
  https://api.github.com/repos/mechmind-dwv/mechmind-dwv/dispatches
