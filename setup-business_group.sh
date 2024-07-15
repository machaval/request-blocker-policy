#!/bin/bash

# Parse the JSON and extract id and name
json=`anypoint-cli-v4 account business-group list -o=json`

# Extract ids and names using jq
ids=($(echo "$json" | jq -r '.[] | .id'))
names=($(echo "$json" | jq -r '.[] | .name'))

# Display options to the user
echo "Please select an organization:"
for i in "${!names[@]}"; do
  echo "$i) ${names[$i]}"
done

# Read user selection
read -p "Enter the number corresponding to your choice: " choice

# Get the selected id
selected_id=${ids[$choice]}


# Replace the content of cargo.toml
cargo_toml="Cargo.toml"
if [ -f "$cargo_toml" ]; then
  cp "$cargo_toml" "cargo_tmp.toml"
  cat cargo_tmp.toml | sed "s/\"group_id_value\"/\"$selected_id\"/" > $cargo_toml
  rm cargo_tmp.toml
  echo "Updated cargo.toml with id: $selected_id"
else
  echo "cargo.toml not found!"
fi