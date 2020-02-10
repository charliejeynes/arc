(function() {var implementors = {};
implementors["nalgebra"] = [{"text":"impl&lt;N, R:&nbsp;<a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimName.html\" title=\"trait nalgebra::base::dimension::DimName\">DimName</a>, C:&nbsp;<a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimName.html\" title=\"trait nalgebra::base::dimension::DimName\">DimName</a>&gt; <a class=\"trait\" href=\"alga/general/identity/trait.Identity.html\" title=\"trait alga::general::identity::Identity\">Identity</a>&lt;<a class=\"struct\" href=\"alga/general/operator/struct.Additive.html\" title=\"struct alga::general::operator::Additive\">Additive</a>&gt; for <a class=\"type\" href=\"nalgebra/base/type.MatrixMN.html\" title=\"type nalgebra::base::MatrixMN\">MatrixMN</a>&lt;N, R, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a> + <a class=\"trait\" href=\"num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"nalgebra/base/default_allocator/struct.DefaultAllocator.html\" title=\"struct nalgebra::base::default_allocator::DefaultAllocator\">DefaultAllocator</a>: <a class=\"trait\" href=\"nalgebra/base/allocator/trait.Allocator.html\" title=\"trait nalgebra::base::allocator::Allocator\">Allocator</a>&lt;N, R, C&gt;,&nbsp;</span>","synthetic":false,"types":["nalgebra::base::alias::MatrixMN"]},{"text":"impl&lt;N, D:&nbsp;<a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimName.html\" title=\"trait nalgebra::base::dimension::DimName\">DimName</a>&gt; <a class=\"trait\" href=\"alga/general/identity/trait.Identity.html\" title=\"trait alga::general::identity::Identity\">Identity</a>&lt;<a class=\"struct\" href=\"alga/general/operator/struct.Multiplicative.html\" title=\"struct alga::general::operator::Multiplicative\">Multiplicative</a>&gt; for <a class=\"type\" href=\"nalgebra/base/type.MatrixN.html\" title=\"type nalgebra::base::MatrixN\">MatrixN</a>&lt;N, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a> + <a class=\"trait\" href=\"num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a> + <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"nalgebra/base/default_allocator/struct.DefaultAllocator.html\" title=\"struct nalgebra::base::default_allocator::DefaultAllocator\">DefaultAllocator</a>: <a class=\"trait\" href=\"nalgebra/base/allocator/trait.Allocator.html\" title=\"trait nalgebra::base::allocator::Allocator\">Allocator</a>&lt;N, D, D&gt;,&nbsp;</span>","synthetic":false,"types":["nalgebra::base::alias::MatrixN"]},{"text":"impl&lt;N:&nbsp;<a class=\"trait\" href=\"nalgebra/trait.RealField.html\" title=\"trait nalgebra::RealField\">RealField</a>, D:&nbsp;<a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimName.html\" title=\"trait nalgebra::base::dimension::DimName\">DimName</a>&gt; <a class=\"trait\" href=\"alga/general/identity/trait.Identity.html\" title=\"trait alga::general::identity::Identity\">Identity</a>&lt;<a class=\"struct\" href=\"alga/general/operator/struct.Multiplicative.html\" title=\"struct alga::general::operator::Multiplicative\">Multiplicative</a>&gt; for <a class=\"struct\" href=\"nalgebra/geometry/struct.Rotation.html\" title=\"struct nalgebra::geometry::Rotation\">Rotation</a>&lt;N, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"nalgebra/base/default_allocator/struct.DefaultAllocator.html\" title=\"struct nalgebra::base::default_allocator::DefaultAllocator\">DefaultAllocator</a>: <a class=\"trait\" href=\"nalgebra/base/allocator/trait.Allocator.html\" title=\"trait nalgebra::base::allocator::Allocator\">Allocator</a>&lt;N, D, D&gt;,&nbsp;</span>","synthetic":false,"types":["nalgebra::geometry::rotation::Rotation"]},{"text":"impl&lt;N:&nbsp;<a class=\"trait\" href=\"nalgebra/trait.RealField.html\" title=\"trait nalgebra::RealField\">RealField</a>&gt; <a class=\"trait\" href=\"alga/general/identity/trait.Identity.html\" title=\"trait alga::general::identity::Identity\">Identity</a>&lt;<a class=\"struct\" href=\"alga/general/operator/struct.Multiplicative.html\" title=\"struct alga::general::operator::Multiplicative\">Multiplicative</a>&gt; for <a class=\"struct\" href=\"nalgebra/geometry/struct.Quaternion.html\" title=\"struct nalgebra::geometry::Quaternion\">Quaternion</a>&lt;N&gt;","synthetic":false,"types":["nalgebra::geometry::quaternion::Quaternion"]},{"text":"impl&lt;N:&nbsp;<a class=\"trait\" href=\"nalgebra/trait.RealField.html\" title=\"trait nalgebra::RealField\">RealField</a>&gt; <a class=\"trait\" href=\"alga/general/identity/trait.Identity.html\" title=\"trait alga::general::identity::Identity\">Identity</a>&lt;<a class=\"struct\" href=\"alga/general/operator/struct.Additive.html\" title=\"struct alga::general::operator::Additive\">Additive</a>&gt; for <a class=\"struct\" href=\"nalgebra/geometry/struct.Quaternion.html\" title=\"struct nalgebra::geometry::Quaternion\">Quaternion</a>&lt;N&gt;","synthetic":false,"types":["nalgebra::geometry::quaternion::Quaternion"]},{"text":"impl&lt;N:&nbsp;<a class=\"trait\" href=\"nalgebra/trait.RealField.html\" title=\"trait nalgebra::RealField\">RealField</a>&gt; <a class=\"trait\" href=\"alga/general/identity/trait.Identity.html\" title=\"trait alga::general::identity::Identity\">Identity</a>&lt;<a class=\"struct\" href=\"alga/general/operator/struct.Multiplicative.html\" title=\"struct alga::general::operator::Multiplicative\">Multiplicative</a>&gt; for <a class=\"type\" href=\"nalgebra/geometry/type.UnitQuaternion.html\" title=\"type nalgebra::geometry::UnitQuaternion\">UnitQuaternion</a>&lt;N&gt;","synthetic":false,"types":["nalgebra::geometry::quaternion::UnitQuaternion"]},{"text":"impl&lt;N:&nbsp;<a class=\"trait\" href=\"nalgebra/trait.RealField.html\" title=\"trait nalgebra::RealField\">RealField</a>&gt; <a class=\"trait\" href=\"alga/general/identity/trait.Identity.html\" title=\"trait alga::general::identity::Identity\">Identity</a>&lt;<a class=\"struct\" href=\"alga/general/operator/struct.Multiplicative.html\" title=\"struct alga::general::operator::Multiplicative\">Multiplicative</a>&gt; for <a class=\"type\" href=\"nalgebra/geometry/type.UnitComplex.html\" title=\"type nalgebra::geometry::UnitComplex\">UnitComplex</a>&lt;N&gt;","synthetic":false,"types":["nalgebra::geometry::unit_complex::UnitComplex"]},{"text":"impl&lt;N:&nbsp;<a class=\"trait\" href=\"nalgebra/trait.RealField.html\" title=\"trait nalgebra::RealField\">RealField</a>, D:&nbsp;<a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimName.html\" title=\"trait nalgebra::base::dimension::DimName\">DimName</a>&gt; <a class=\"trait\" href=\"alga/general/identity/trait.Identity.html\" title=\"trait alga::general::identity::Identity\">Identity</a>&lt;<a class=\"struct\" href=\"alga/general/operator/struct.Multiplicative.html\" title=\"struct alga::general::operator::Multiplicative\">Multiplicative</a>&gt; for <a class=\"struct\" href=\"nalgebra/geometry/struct.Translation.html\" title=\"struct nalgebra::geometry::Translation\">Translation</a>&lt;N, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"nalgebra/base/default_allocator/struct.DefaultAllocator.html\" title=\"struct nalgebra::base::default_allocator::DefaultAllocator\">DefaultAllocator</a>: <a class=\"trait\" href=\"nalgebra/base/allocator/trait.Allocator.html\" title=\"trait nalgebra::base::allocator::Allocator\">Allocator</a>&lt;N, D&gt;,&nbsp;</span>","synthetic":false,"types":["nalgebra::geometry::translation::Translation"]},{"text":"impl&lt;N:&nbsp;<a class=\"trait\" href=\"nalgebra/trait.RealField.html\" title=\"trait nalgebra::RealField\">RealField</a>, D:&nbsp;<a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimName.html\" title=\"trait nalgebra::base::dimension::DimName\">DimName</a>, R&gt; <a class=\"trait\" href=\"alga/general/identity/trait.Identity.html\" title=\"trait alga::general::identity::Identity\">Identity</a>&lt;<a class=\"struct\" href=\"alga/general/operator/struct.Multiplicative.html\" title=\"struct alga::general::operator::Multiplicative\">Multiplicative</a>&gt; for <a class=\"struct\" href=\"nalgebra/geometry/struct.Isometry.html\" title=\"struct nalgebra::geometry::Isometry\">Isometry</a>&lt;N, D, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: <a class=\"trait\" href=\"alga/linear/transformation/trait.Rotation.html\" title=\"trait alga::linear::transformation::Rotation\">Rotation</a>&lt;<a class=\"struct\" href=\"nalgebra/geometry/struct.Point.html\" title=\"struct nalgebra::geometry::Point\">Point</a>&lt;N, D&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"nalgebra/base/default_allocator/struct.DefaultAllocator.html\" title=\"struct nalgebra::base::default_allocator::DefaultAllocator\">DefaultAllocator</a>: <a class=\"trait\" href=\"nalgebra/base/allocator/trait.Allocator.html\" title=\"trait nalgebra::base::allocator::Allocator\">Allocator</a>&lt;N, D&gt;,&nbsp;</span>","synthetic":false,"types":["nalgebra::geometry::isometry::Isometry"]},{"text":"impl&lt;N:&nbsp;<a class=\"trait\" href=\"nalgebra/trait.RealField.html\" title=\"trait nalgebra::RealField\">RealField</a>, D:&nbsp;<a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimName.html\" title=\"trait nalgebra::base::dimension::DimName\">DimName</a>, R&gt; <a class=\"trait\" href=\"alga/general/identity/trait.Identity.html\" title=\"trait alga::general::identity::Identity\">Identity</a>&lt;<a class=\"struct\" href=\"alga/general/operator/struct.Multiplicative.html\" title=\"struct alga::general::operator::Multiplicative\">Multiplicative</a>&gt; for <a class=\"struct\" href=\"nalgebra/geometry/struct.Similarity.html\" title=\"struct nalgebra::geometry::Similarity\">Similarity</a>&lt;N, D, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: <a class=\"trait\" href=\"alga/linear/transformation/trait.Rotation.html\" title=\"trait alga::linear::transformation::Rotation\">Rotation</a>&lt;<a class=\"struct\" href=\"nalgebra/geometry/struct.Point.html\" title=\"struct nalgebra::geometry::Point\">Point</a>&lt;N, D&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"nalgebra/base/default_allocator/struct.DefaultAllocator.html\" title=\"struct nalgebra::base::default_allocator::DefaultAllocator\">DefaultAllocator</a>: <a class=\"trait\" href=\"nalgebra/base/allocator/trait.Allocator.html\" title=\"trait nalgebra::base::allocator::Allocator\">Allocator</a>&lt;N, D&gt;,&nbsp;</span>","synthetic":false,"types":["nalgebra::geometry::similarity::Similarity"]},{"text":"impl&lt;N:&nbsp;<a class=\"trait\" href=\"nalgebra/trait.RealField.html\" title=\"trait nalgebra::RealField\">RealField</a>, D:&nbsp;<a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimNameAdd.html\" title=\"trait nalgebra::base::dimension::DimNameAdd\">DimNameAdd</a>&lt;<a class=\"struct\" href=\"nalgebra/base/dimension/struct.U1.html\" title=\"struct nalgebra::base::dimension::U1\">U1</a>&gt;, C&gt; <a class=\"trait\" href=\"alga/general/identity/trait.Identity.html\" title=\"trait alga::general::identity::Identity\">Identity</a>&lt;<a class=\"struct\" href=\"alga/general/operator/struct.Multiplicative.html\" title=\"struct alga::general::operator::Multiplicative\">Multiplicative</a>&gt; for <a class=\"struct\" href=\"nalgebra/geometry/struct.Transform.html\" title=\"struct nalgebra::geometry::Transform\">Transform</a>&lt;N, D, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"nalgebra/geometry/trait.TCategory.html\" title=\"trait nalgebra::geometry::TCategory\">TCategory</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"nalgebra/base/default_allocator/struct.DefaultAllocator.html\" title=\"struct nalgebra::base::default_allocator::DefaultAllocator\">DefaultAllocator</a>: <a class=\"trait\" href=\"nalgebra/base/allocator/trait.Allocator.html\" title=\"trait nalgebra::base::allocator::Allocator\">Allocator</a>&lt;N, <a class=\"type\" href=\"nalgebra/base/dimension/type.DimNameSum.html\" title=\"type nalgebra::base::dimension::DimNameSum\">DimNameSum</a>&lt;D, <a class=\"struct\" href=\"nalgebra/base/dimension/struct.U1.html\" title=\"struct nalgebra::base::dimension::U1\">U1</a>&gt;, <a class=\"type\" href=\"nalgebra/base/dimension/type.DimNameSum.html\" title=\"type nalgebra::base::dimension::DimNameSum\">DimNameSum</a>&lt;D, <a class=\"struct\" href=\"nalgebra/base/dimension/struct.U1.html\" title=\"struct nalgebra::base::dimension::U1\">U1</a>&gt;&gt;,&nbsp;</span>","synthetic":false,"types":["nalgebra::geometry::transform::Transform"]}];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        })()