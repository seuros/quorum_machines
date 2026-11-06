# frozen_string_literal: true
require_relative 'quorum_machines/version'
require 'quorum_machines_native/quorum_machines_native'

module QuorumMachines
  def self.intersects?(members:, reads:, writes:) = _intersects(members, reads, writes)
  private_class_method :_intersects

  class Quorum
    def self.new(members:, required:) = _new(members, required)
    private_class_method :_new
  end
end
