#!/usr/bin/env bash

set -ex

# ./run.sh 2.5-alpine 9999 "sleep 3"

version="${1:-2.5-alpine}"
stats_socket_port=$2
callback=$3

if [ -z "$stats_socket_port" ]
then
    exit 91
fi
if [ -z "$callback" ]
then
    exit 92
fi

script_path=$(cd $(dirname $0) ; pwd -P)
script_path_root="${script_path}/"

# 
container_name="haproxy_with_stats_socket_${stats_socket_port}"

conf_file="${script_path_root}conf/haproxy.cfg"
run_dir="${script_path_root}run"
sock_file="${script_path_root}run/haproxy.sock"

sudo chmod 777 "${run_dir}"
rm -rf "${sock_file}"

cleanup() {
    docker stop ${container_name}

    sleep 1

    test -S "${sock_file}" && rm -rf "${sock_file}"
}
trap cleanup EXIT

# 
# https://www.consul.io/docs/install/ports
# 
# https://unix.stackexchange.com/questions/55913/whats-the-easiest-way-to-find-an-unused-local-port
read LOWERPORT UPPERPORT < /proc/sys/net/ipv4/ip_local_port_range
http_frontend_port=$(comm -23 <(seq $LOWERPORT $UPPERPORT | sort) <(ss -Htan | awk '{print $4}' | cut -d':' -f2 | sort -u) | shuf | head -n 1)
tcp_frontend_port=$(comm -23 <(seq $LOWERPORT $UPPERPORT | sort) <(ss -Htan | awk '{print $4}' | cut -d':' -f2 | sort -u) | shuf | head -n 1)
http_listen_port=$(comm -23 <(seq $LOWERPORT $UPPERPORT | sort) <(ss -Htan | awk '{print $4}' | cut -d':' -f2 | sort -u) | shuf | head -n 1)
tcp_listen_port=$(comm -23 <(seq $LOWERPORT $UPPERPORT | sort) <(ss -Htan | awk '{print $4}' | cut -d':' -f2 | sort -u) | shuf | head -n 1)

docker run -d --rm --name ${container_name} \
    -e HAPROXY_STATS_SOCKET_PORT=${stats_socket_port} \
    -e HAPROXY_HTTP_FRONTEND_PORT=${http_frontend_port} \
    -e HAPROXY_TCP_FRONTEND_PORT=${tcp_frontend_port} \
    -e HAPROXY_HTTP_LISTEN_PORT=${http_listen_port} \
    -e HAPROXY_TCP_LISTEN_PORT=${tcp_listen_port} \
    -v "${conf_file}":/usr/local/etc/haproxy/haproxy.cfg \
    -v "${run_dir}":/var/run \
    -p ${stats_socket_port}:${stats_socket_port}\
    haproxy:${version} \
    haproxy -f /usr/local/etc/haproxy/haproxy.cfg

sleep 1

if [ -x "$(command -v socat)" ]; then
    echo "show info" | sudo socat "${sock_file}" stdio
    echo "show stat" | socat TCP4:127.0.0.1:${stats_socket_port} stdio
fi

# 
echo "callback running..."
bash -c "${callback}"
