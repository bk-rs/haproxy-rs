## Files

```
./haproxy_docker/with_stats_socket/run.sh 2.5.5-alpine 9255 "sleep 1000"
```

```
echo "show env" | socat TCP4:127.0.0.1:9255 stdio > ./haproxy-stats/tests/files/2_5_5_show_env.txt

echo "show info" | socat TCP4:127.0.0.1:9255 stdio > ./haproxy-stats/tests/files/2_5_5_show_info.txt

echo "show info json" | socat TCP4:127.0.0.1:9255 stdio > ./haproxy-stats/tests/files/2_5_5_show_info.json

echo "show stat" | socat TCP4:127.0.0.1:9255 stdio > ./haproxy-stats/tests/files/2_5_5_show_stat.csv

echo "show stat json" | socat TCP4:127.0.0.1:9255 stdio > ./haproxy-stats/tests/files/2_5_5_show_stat.json
```
