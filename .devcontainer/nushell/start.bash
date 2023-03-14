#!/usr/bin/env bash

nu -c "zellij attach --create options --pane-frames false --default-shell (which nu |get 0 |get path)"
exit
