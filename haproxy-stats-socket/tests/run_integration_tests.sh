#!/usr/bin/env bash

set -ex

version="${1:-2.5-alpine}"

# RUST_BACKTRACE=full ./tests/run_integration_tests.sh
# RUST_LOG=trace ./tests/run_integration_tests.sh

script_path=$(cd $(dirname $0) ; pwd -P)
script_path_root="${script_path}/"

run="${script_path_root}../../haproxy_docker/with_stats_socket/run.sh"

# https://unix.stackexchange.com/questions/55913/whats-the-easiest-way-to-find-an-unused-local-port
read LOWERPORT UPPERPORT < /proc/sys/net/ipv4/ip_local_port_range
stats_socket_port=$(comm -23 <(seq $LOWERPORT $UPPERPORT | sort) <(ss -Htan | awk '{print $4}' | cut -d':' -f2 | sort -u) | shuf | head -n 1)

export HAPROXY_STATS_SOCKET_TCP_PORT="${stats_socket_port}"
export HAPROXY_STATS_SOCKET_UNIX_PATH="${script_path_root}run/haproxy.sock"

${run} ${version} ${stats_socket_port} "cd ${script_path_root}..; cargo test -p haproxy-stats-socket --features _integration_tests -- --nocapture"
