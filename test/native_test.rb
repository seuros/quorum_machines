# frozen_string_literal: true
require 'test_helper'

class QuorumNativeTest < Minitest::Test
  def test_distinct_votes_threshold_and_reset
    quorum = QuorumMachines::Quorum.new(members: 3, required: 2)
    assert quorum.acknowledge(0)
    assert quorum.acknowledge(0)
    assert_equal 1, quorum.count
    refute quorum.reached?
    refute quorum.acknowledge(3)
    assert quorum.acknowledge(2)
    assert quorum.reached?
    quorum.reset
    assert_equal 0, quorum.count
    refute quorum.reached?
  end

  def test_membership_bounds_and_integer_validation
    [[0, 1], [65, 2], [2, 3], [3, 0]].each do |members, required|
      assert_raises(ArgumentError) { QuorumMachines::Quorum.new(members: members, required: required) }
    end
    assert_raises(TypeError) { QuorumMachines::Quorum.new(members: 3.0, required: 2) }
    quorum = QuorumMachines::Quorum.new(members: 3, required: 2)
    assert_raises(TypeError) { quorum.acknowledge(1.0) }
    assert_raises(RangeError) { quorum.acknowledge(-1) }
    quorum.freeze
    assert_raises(FrozenError) { quorum.acknowledge(1) }
    assert_raises(FrozenError) { quorum.reset }
  end

  def test_threshold_intersection
    assert QuorumMachines.intersects?(members: 3, reads: 2, writes: 2)
    refute QuorumMachines.intersects?(members: 3, reads: 1, writes: 2)
    refute QuorumMachines.intersects?(members: 3, reads: 4, writes: 2)
  end

  def test_randomized_membership_matches_a_distinct_vote_reference
    random = Random.new(4216)
    [1, 3, 5, 17, 64].each do |members|
      required = (members / 2) + 1
      quorum = QuorumMachines::Quorum.new(members: members, required: required)
      seen = []
      100.times do
        member = random.rand(0..members)
        assert_equal member < members, quorum.acknowledge(member)
        seen << member if member < members && !seen.include?(member)
        assert_equal seen.length, quorum.count
        assert_equal seen.length >= required, quorum.reached?
      end
      GC.compact
      assert_equal seen.length, quorum.count
    end
  end
end
