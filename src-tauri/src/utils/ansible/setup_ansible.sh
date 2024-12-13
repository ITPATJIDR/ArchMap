#!/bin/bash
# Setup Ansible environment
python3 -m virtualenv ansible_env
source ansible_env/bin/activate
pip install ansible
