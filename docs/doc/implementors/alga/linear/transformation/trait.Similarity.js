(function() {var implementors = {};
implementors["nalgebra"] = [{"text":"impl&lt;N:&nbsp;<a class=\"trait\" href=\"nalgebra/trait.RealField.html\" title=\"trait nalgebra::RealField\">RealField</a>, D:&nbsp;<a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimName.html\" title=\"trait nalgebra::base::dimension::DimName\">DimName</a>&gt; <a class=\"trait\" href=\"alga/linear/transformation/trait.Similarity.html\" title=\"trait alga::linear::transformation::Similarity\">Similarity</a>&lt;<a class=\"struct\" href=\"nalgebra/geometry/struct.Point.html\" title=\"struct nalgebra::geometry::Point\">Point</a>&lt;N, D&gt;&gt; for <a class=\"struct\" href=\"nalgebra/geometry/struct.Rotation.html\" title=\"struct nalgebra::geometry::Rotation\">Rotation</a>&lt;N, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"nalgebra/base/default_allocator/struct.DefaultAllocator.html\" title=\"struct nalgebra::base::default_allocator::DefaultAllocator\">DefaultAllocator</a>: <a class=\"trait\" href=\"nalgebra/base/allocator/trait.Allocator.html\" title=\"trait nalgebra::base::allocator::Allocator\">Allocator</a>&lt;N, D, D&gt; + <a class=\"trait\" href=\"nalgebra/base/allocator/trait.Allocator.html\" title=\"trait nalgebra::base::allocator::Allocator\">Allocator</a>&lt;N, D&gt;,&nbsp;</span>","synthetic":false,"types":["nalgebra::geometry::rotation::Rotation"]},{"text":"impl&lt;N:&nbsp;<a class=\"trait\" href=\"nalgebra/trait.RealField.html\" title=\"trait nalgebra::RealField\">RealField</a>&gt; <a class=\"trait\" href=\"alga/linear/transformation/trait.Similarity.html\" title=\"trait alga::linear::transformation::Similarity\">Similarity</a>&lt;<a class=\"struct\" href=\"nalgebra/geometry/struct.Point.html\" title=\"struct nalgebra::geometry::Point\">Point</a>&lt;N, <a class=\"struct\" href=\"nalgebra/base/dimension/struct.U3.html\" title=\"struct nalgebra::base::dimension::U3\">U3</a>&gt;&gt; for <a class=\"type\" href=\"nalgebra/geometry/type.UnitQuaternion.html\" title=\"type nalgebra::geometry::UnitQuaternion\">UnitQuaternion</a>&lt;N&gt;","synthetic":false,"types":["nalgebra::geometry::quaternion::UnitQuaternion"]},{"text":"impl&lt;N:&nbsp;<a class=\"trait\" href=\"nalgebra/trait.RealField.html\" title=\"trait nalgebra::RealField\">RealField</a>&gt; <a class=\"trait\" href=\"alga/linear/transformation/trait.Similarity.html\" title=\"trait alga::linear::transformation::Similarity\">Similarity</a>&lt;<a class=\"struct\" href=\"nalgebra/geometry/struct.Point.html\" title=\"struct nalgebra::geometry::Point\">Point</a>&lt;N, <a class=\"struct\" href=\"nalgebra/base/dimension/struct.U2.html\" title=\"struct nalgebra::base::dimension::U2\">U2</a>&gt;&gt; for <a class=\"type\" href=\"nalgebra/geometry/type.UnitComplex.html\" title=\"type nalgebra::geometry::UnitComplex\">UnitComplex</a>&lt;N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"nalgebra/base/default_allocator/struct.DefaultAllocator.html\" title=\"struct nalgebra::base::default_allocator::DefaultAllocator\">DefaultAllocator</a>: <a class=\"trait\" href=\"nalgebra/base/allocator/trait.Allocator.html\" title=\"trait nalgebra::base::allocator::Allocator\">Allocator</a>&lt;N, <a class=\"struct\" href=\"nalgebra/base/dimension/struct.U2.html\" title=\"struct nalgebra::base::dimension::U2\">U2</a>&gt;,&nbsp;</span>","synthetic":false,"types":["nalgebra::geometry::unit_complex::UnitComplex"]},{"text":"impl&lt;N:&nbsp;<a class=\"trait\" href=\"nalgebra/trait.RealField.html\" title=\"trait nalgebra::RealField\">RealField</a>, D:&nbsp;<a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimName.html\" title=\"trait nalgebra::base::dimension::DimName\">DimName</a>&gt; <a class=\"trait\" href=\"alga/linear/transformation/trait.Similarity.html\" title=\"trait alga::linear::transformation::Similarity\">Similarity</a>&lt;<a class=\"struct\" href=\"nalgebra/geometry/struct.Point.html\" title=\"struct nalgebra::geometry::Point\">Point</a>&lt;N, D&gt;&gt; for <a class=\"struct\" href=\"nalgebra/geometry/struct.Translation.html\" title=\"struct nalgebra::geometry::Translation\">Translation</a>&lt;N, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"nalgebra/base/default_allocator/struct.DefaultAllocator.html\" title=\"struct nalgebra::base::default_allocator::DefaultAllocator\">DefaultAllocator</a>: <a class=\"trait\" href=\"nalgebra/base/allocator/trait.Allocator.html\" title=\"trait nalgebra::base::allocator::Allocator\">Allocator</a>&lt;N, D&gt;,&nbsp;</span>","synthetic":false,"types":["nalgebra::geometry::translation::Translation"]},{"text":"impl&lt;N:&nbsp;<a class=\"trait\" href=\"nalgebra/trait.RealField.html\" title=\"trait nalgebra::RealField\">RealField</a>, D:&nbsp;<a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimName.html\" title=\"trait nalgebra::base::dimension::DimName\">DimName</a>, R&gt; <a class=\"trait\" href=\"alga/linear/transformation/trait.Similarity.html\" title=\"trait alga::linear::transformation::Similarity\">Similarity</a>&lt;<a class=\"struct\" href=\"nalgebra/geometry/struct.Point.html\" title=\"struct nalgebra::geometry::Point\">Point</a>&lt;N, D&gt;&gt; for <a class=\"struct\" href=\"nalgebra/geometry/struct.Isometry.html\" title=\"struct nalgebra::geometry::Isometry\">Isometry</a>&lt;N, D, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: <a class=\"trait\" href=\"alga/linear/transformation/trait.Rotation.html\" title=\"trait alga::linear::transformation::Rotation\">Rotation</a>&lt;<a class=\"struct\" href=\"nalgebra/geometry/struct.Point.html\" title=\"struct nalgebra::geometry::Point\">Point</a>&lt;N, D&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"nalgebra/base/default_allocator/struct.DefaultAllocator.html\" title=\"struct nalgebra::base::default_allocator::DefaultAllocator\">DefaultAllocator</a>: <a class=\"trait\" href=\"nalgebra/base/allocator/trait.Allocator.html\" title=\"trait nalgebra::base::allocator::Allocator\">Allocator</a>&lt;N, D&gt;,&nbsp;</span>","synthetic":false,"types":["nalgebra::geometry::isometry::Isometry"]},{"text":"impl&lt;N:&nbsp;<a class=\"trait\" href=\"nalgebra/trait.RealField.html\" title=\"trait nalgebra::RealField\">RealField</a>, D:&nbsp;<a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimName.html\" title=\"trait nalgebra::base::dimension::DimName\">DimName</a>, R&gt; <a class=\"trait\" href=\"alga/linear/transformation/trait.Similarity.html\" title=\"trait alga::linear::transformation::Similarity\">Similarity</a>&lt;<a class=\"struct\" href=\"nalgebra/geometry/struct.Point.html\" title=\"struct nalgebra::geometry::Point\">Point</a>&lt;N, D&gt;&gt; for <a class=\"struct\" href=\"nalgebra/geometry/struct.Similarity.html\" title=\"struct nalgebra::geometry::Similarity\">Similarity</a>&lt;N, D, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: <a class=\"trait\" href=\"alga/linear/transformation/trait.Rotation.html\" title=\"trait alga::linear::transformation::Rotation\">Rotation</a>&lt;<a class=\"struct\" href=\"nalgebra/geometry/struct.Point.html\" title=\"struct nalgebra::geometry::Point\">Point</a>&lt;N, D&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"nalgebra/base/default_allocator/struct.DefaultAllocator.html\" title=\"struct nalgebra::base::default_allocator::DefaultAllocator\">DefaultAllocator</a>: <a class=\"trait\" href=\"nalgebra/base/allocator/trait.Allocator.html\" title=\"trait nalgebra::base::allocator::Allocator\">Allocator</a>&lt;N, D&gt;,&nbsp;</span>","synthetic":false,"types":["nalgebra::geometry::similarity::Similarity"]}];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        })()