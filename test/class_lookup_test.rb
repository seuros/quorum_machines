# frozen_string_literal: true

require 'test_helper'
require 'open3'
require 'rbconfig'

# The class of the value the extension wraps (Quorum) is fixed when it loads.
# Nothing a caller does later, redefining the constant or
# `Object.const_get`, reaches a quorum being built. Each case runs in a fresh
# process: the class is fixed once per process, at load.
class ClassLookupTest < Minitest::Test
  LIB = File.expand_path('../lib', __dir__)

  PRELUDE = <<~RUBY
    require 'quorum_machines'
    Q = QuorumMachines
  RUBY

  # Runs `body` after PRELUDE in a new Ruby; answers its last line of output.
  def isolated(body)
    out, err, status = Open3.capture3(RbConfig.ruby, '-I', LIB, '-e', PRELUDE + body)
    assert status.success?, "exited #{status.exitstatus}: #{err}"
    out.lines.last&.chomp
  end

  # Bug: a Quorum constant redefined before the first quorum made `new` and
  # `majority` answer an instance of the new class, with none of the quorum's
  # methods.
  def test_a_redefined_quorum_constant_does_not_change_the_quorum_class
    assert_equal 'true true counted true', isolated(<<~RUBY)
      original = Q::Quorum
      Q.send(:remove_const, :Quorum)
      Q.const_set(:Quorum, Class.new)
      quorum = original.new(1, 1)
      majority = original.majority(3)
      puts [quorum.instance_of?(original), majority.instance_of?(original), quorum.acknowledge(0),
            quorum.reached?].join(' ')
    RUBY
  end

  # Bug: building the first quorum ran `Object.const_get`, a caller's code in
  # the middle of a native call. No Ruby runs while a quorum is built now.
  def test_building_a_quorum_runs_no_const_get
    assert_equal '0 Quorum Quorum', isolated(<<~RUBY)
      hits = 0
      Object.define_singleton_method(:const_get) do |*args|
        hits += 1
        super(*args)
      end
      built = [Q::Quorum.majority(5), Q::Quorum.new(3, 2)]
      puts [hits, *built.map { it.class.name.split('::').last }].join(' ')
    RUBY
  end

  # Bug: a redefined `Object.const_get` that raised hit an `unwrap` in the
  # extension: an uncatchable fatal, and the process exited.
  def test_a_raising_const_get_does_not_kill_the_process
    assert_equal 'counted 2', isolated(<<~RUBY)
      Object.define_singleton_method(:const_get) { |*| raise IOError, 'lookup ran mid-call' }
      quorum = Q::Quorum.majority(3)
      puts [quorum.acknowledge(1), quorum.required].join(' ')
    RUBY
  end
end
