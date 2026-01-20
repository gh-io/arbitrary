cargo clean
sccache --show-stats
cargo build
sccache --show-stats
apt-get install build-essential autoconf libssl-dev libyaml-dev zlib1g-dev libffi-dev libgmp-dev rustc
gem install google-apis-drive_v3 google-apis-calendar_v3
bundle install
gem build google-apis-drive_v3.gemspec
gem install ./google-apis-drive_v3-0.x.x.gem
ruby -e "require 'google/apis/drive_v3'; require 'google/apis/calendar_v3'; puts 'Success!'"
