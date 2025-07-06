#!/bin/bash
if [ $1 == "-u" ]; then
    cp -f test_echo.service /home/dev/.config/systemd/user/test_echo.service
    systemctl --user daemon-reexec
    systemctl --user daemon-reload
    systemctl --user enable --now test_echo.service
else
    cp -f test_echo.service /etc/systemd/system//test_echo.service
fi
chmod +x test_echo.sh


