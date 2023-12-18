// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/blueprint/components/primary_weights.fbs".

#pragma once

#include "../../collection.hpp"
#include "../../result.hpp"

#include <cstdint>
#include <memory>
#include <utility>

namespace arrow {
    class Array;
    class DataType;
    class ListBuilder;
} // namespace arrow

namespace rerun::blueprint::components {
    /// **Component**: The weights of the primary axis.
    ///
    /// For `Grid` this is the column weights.
    struct PrimaryWeights {
        /// The weighting of each container element.
        rerun::Collection<float> weights;

      public:
        PrimaryWeights() = default;

        PrimaryWeights(rerun::Collection<float> weights_) : weights(std::move(weights_)) {}

        PrimaryWeights& operator=(rerun::Collection<float> weights_) {
            weights = std::move(weights_);
            return *this;
        }
    };
} // namespace rerun::blueprint::components

namespace rerun {
    template <typename T>
    struct Loggable;

    /// \private
    template <>
    struct Loggable<blueprint::components::PrimaryWeights> {
        static constexpr const char Name[] = "rerun.blueprint.components.PrimaryWeights";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::ListBuilder* builder, const blueprint::components::PrimaryWeights* elements,
            size_t num_elements
        );

        /// Serializes an array of `rerun::blueprint:: components::PrimaryWeights` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const blueprint::components::PrimaryWeights* instances, size_t num_instances
        );
    };
} // namespace rerun
