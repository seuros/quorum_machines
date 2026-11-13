# frozen_string_literal: true
require 'test_helper'

class QuorumNativeTest < Minitest::Test
  Q = QuorumMachines::Quorum

  def test_distinct_votes_threshold_and_reset
    quorum = Q.new(3, 2)
    assert_equal [3, 2, 0], [quorum.members, quorum.required, quorum.count]
    assert_equal :counted, quorum.acknowledge(0)
    assert_equal :duplicate, quorum.acknowledge(0)
    assert_equal 1, quorum.count
    refute quorum.reached?
    assert_equal :unknown_member, quorum.acknowledge(3)
    assert_equal :counted, quorum.acknowledge(2)
    assert quorum.reached?
    assert quorum.acknowledged?(2)
    refute quorum.acknowledged?(1)
    refute quorum.acknowledged?(3)
    assert_nil quorum.reset
    assert_equal 0, quorum.count
    refute quorum.reached?
    refute quorum.acknowledged?(2)
  end

  def test_majority_mirrors_the_core
    assert_equal 64, Q::CAPACITY
    { 1 => 1, 2 => 2, 3 => 2, 4 => 3, 5 => 3, 64 => 33 }.each do |members, required|
      quorum = Q.majority(members)
      assert_equal [members, required], [quorum.members, quorum.required]
    end
    [0, 65].each { |members| assert_raises(RangeError) { Q.majority(members) } }
    assert_raises(TypeError) { Q.majority(3.0) }
  end

  def test_a_refusal_or_a_repeat_changes_nothing
    quorum = Q.new(3, 3)
    quorum.acknowledge(1)
    before = snapshot(quorum)
    assert_equal :unknown_member, quorum.acknowledge(3)
    assert_equal :unknown_member, quorum.acknowledge(63)
    assert_equal :unknown_member, quorum.acknowledge(2**40)
    assert_equal :duplicate, quorum.acknowledge(1)
    assert_equal before, snapshot(quorum)
  end

  def test_configuration_bounds_integer_validation_and_freezing
    [[0, 1], [65, 2], [2, 3], [3, 0], [0, 0]].each do |members, required|
      assert_raises(RangeError) { Q.new(members, required) }
    end
    assert_raises(TypeError) { Q.new(3.0, 2) }
    assert_raises(TypeError) { Q.new(3, nil) }
    assert_raises(ArgumentError) { Q.new(3) }
    assert_raises(ArgumentError) { Q.new(members: 3, required: 2) }
    assert_raises(TypeError) { Q.allocate }
    quorum = Q.new(3, 2)
    assert_raises(TypeError) { quorum.acknowledge(1.0) }
    assert_raises(RangeError) { quorum.acknowledge(-1) }
    assert_raises(TypeError) { quorum.acknowledged?('1') }
    quorum.freeze
    assert_raises(FrozenError) { quorum.acknowledge(1) }
    assert_raises(FrozenError) { quorum.reset }
    assert_equal 0, quorum.count
  end

  def test_threshold_intersection
    assert QuorumMachines.intersects?(3, 2, 2)
    refute QuorumMachines.intersects?(3, 1, 2)
    refute QuorumMachines.intersects?(3, 4, 2)
    refute QuorumMachines.intersects?(0, 0, 0)
    assert_raises(ArgumentError) { QuorumMachines.intersects?(members: 3, reads: 2, writes: 2) }
    assert_raises(TypeError) { QuorumMachines.intersects?(3, 2.0, 2) }
  end

  def test_randomized_membership_matches_a_distinct_vote_reference
    random = Random.new(4216)
    [1, 3, 5, 17, 64].each do |members|
      quorum = Q.majority(members)
      seen = []
      100.times do
        member = random.rand(0..members + 2)
        before = snapshot(quorum)
        answer = quorum.acknowledge(member)
        if member >= members
          assert_equal :unknown_member, answer
          assert_equal before, snapshot(quorum)
        elsif seen.include?(member)
          assert_equal :duplicate, answer
          assert_equal before, snapshot(quorum)
        else
          assert_equal :counted, answer
          seen << member
        end
        assert_equal seen.length, quorum.count
        assert_equal seen.length >= quorum.required, quorum.reached?
      end
      GC.compact
      assert_equal seen.length, quorum.count
    end
  end

  private

  def snapshot(quorum)
    [quorum.count, quorum.reached?, (0...Q::CAPACITY).map { |member| quorum.acknowledged?(member) }]
  end
end
