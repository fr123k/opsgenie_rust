Vagrant.configure("2") do |config|
    config.vm.box = "fr123k/ubuntu21-rust"
    # config.vm.box_version = "1.0.0"
    config.ssh.extra_args = ["-t", "cd /vagrant; mkdir -p /tmp/vagrant/target; ln -s /tmp/vagrant/target/ ./target; bash --login"]
end