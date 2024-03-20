#!/bin/bash
# ================================================================
#  华北160服务器开机自动启动程序                     ---临来笑笑生
# ================================================================

### BEGIN INIT INFO
# Provides:          160.SH
# Required-Start:    $remote_fs $syslog
# Required-Stop:     $remote_fs $syslog
# Default-Start:     2 3 4 5
# Default-Stop:      0 1 6
# Short-Description: gwmc.vip+Nginx+PHP+mysql一键启动
# Description:       本人比较懒，就不写了
### END INIT INFO

# http://gwmc.vip
cd /var/www/www.gwmc.vip
nohup ./target/release/www-gwmc-vip &

# http://manage.gwmc.vip
cd /var/www/manage.gwmc.vip
nohup ./target/release/manage-gwmc-vip &

# 启动nginx
/usr/local/nginx/nginx