// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/components/half_sizes2d.fbs".

#pragma once

#include "../data_cell.hpp"
#include "../datatypes/vec2d.hpp"
#include "../result.hpp"

#include <cstdint>
#include <memory>
#include <utility>

namespace arrow {
    class DataType;
    class FixedSizeListBuilder;
    class MemoryPool;
} // namespace arrow

namespace rerun {
    namespace components {
        /// **Component**: Half-sizes (extents) of a 2D box along its local axis, starting at its
        /// local origin/center.
        ///
        /// The box extends both in negative and positive direction along each axis.
        /// Negative sizes indicate that the box is flipped along the respective axis, but this has
        /// no effect on how it is displayed.
        struct HalfSizes2D {
            rerun::datatypes::Vec2D xy;

            /// Name of the component, used for serialization.
            static const char NAME[];

          public:
            // Extensions to generated type defined in 'half_sizes2d_ext.cpp'

            /// Construct HalfSizes2D from x/y values.
            HalfSizes2D(float x, float y) : xy{x, y} {}

            float x() const {
                return xy.x();
            }

            float y() const {
                return xy.y();
            }

          public:
            HalfSizes2D() = default;

            HalfSizes2D(rerun::datatypes::Vec2D _xy) : xy(std::move(_xy)) {}

            HalfSizes2D& operator=(rerun::datatypes::Vec2D _xy) {
                xy = std::move(_xy);
                return *this;
            }

            HalfSizes2D(const float (&arg)[2]) : xy(arg) {}

            /// Returns the arrow data type this type corresponds to.
            static const std::shared_ptr<arrow::DataType>& arrow_datatype();

            /// Creates a new array builder with an array of this type.
            static Result<std::shared_ptr<arrow::FixedSizeListBuilder>> new_arrow_array_builder(
                arrow::MemoryPool* memory_pool
            );

            /// Fills an arrow array builder with an array of this type.
            static Error fill_arrow_array_builder(
                arrow::FixedSizeListBuilder* builder, const HalfSizes2D* elements,
                size_t num_elements
            );

            /// Creates a Rerun DataCell from an array of HalfSizes2D components.
            static Result<rerun::DataCell> to_data_cell(
                const HalfSizes2D* instances, size_t num_instances
            );
        };
    } // namespace components
} // namespace rerun
