use distro_or_package_manager.nu [get_distro_or_package_manager]

export def run [os: string, arch: string, tool_name: string, action_name: string] {
  let script_data: record = (nu -c $"use tools/($tool_name)/data.nu; $data_file_path")
                                    | get "struct"
                                    | get $os
                                    | get (distro_or_package_manager $os)
                                    | get $arch
  let entry = ($script_data | get $action_name)
  match ($entry | describe | get type) {
      "closure" => {
          ($entry)
      },
      "record" => {
          let deps = (entry | get "deps")
          # TODO: ensure deps here
          ($entry | get "command")
      }
  }
}