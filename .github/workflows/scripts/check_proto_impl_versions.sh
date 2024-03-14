#!/bin/sh

proto_files=$(find proto -name '*.proto')
exit_status=0
for proto_file in $proto_files; do
	proto_def_version=$(head -n1 "$proto_file" | sed -n 's/\/\/ Protocol version \([0-9\.]*\)/\1/p')
	if [ -z "$proto_def_version" ]; then
		>&2 echo "[${proto_file}]: No protocol version defined. To ensure consistency, please prepend the following line to ${proto_file}:"
		>&2 echo "// Protocol version PROTOCOL_VERSION"
		continue
	fi

	snake_case_proto_name=$(basename "$proto_file" .proto | sed 's/-/_/g')
	server_impl_version=$(find src -name "${snake_case_proto_name}.rs" -exec grep PROTOCOL_VERSION {} + | sed -n 's/.*"\([0-9\.]*\)".*/\1/p' | head -n 1)
	if [ -z "$server_impl_version" ]; then
		>&2 echo "[${proto_file}]: No server implementation or no server protocol version explicitely defined. Skipping."
		continue
	fi

	if [ "$proto_def_version" != "$server_impl_version" ]; then
		>&2 echo "[${proto_file}]: Server protocol version (${server_impl_version}) differs from definition's (${proto_def_version})"
		exit_status=1
	fi
done

exit $exit_status