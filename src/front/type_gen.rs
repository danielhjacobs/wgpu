/*!
Type generators.
*/

use crate::{arena::Handle, span::Span};

impl crate::Module {
    pub fn generate_atomic_compare_exchange_result(
        &mut self,
        kind: crate::ScalarKind,
        width: crate::Bytes,
    ) -> Handle<crate::Type> {
        let bool_ty = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Scalar {
                    kind: crate::ScalarKind::Bool,
                    width: crate::BOOL_WIDTH,
                },
            },
            Span::UNDEFINED,
        );
        let scalar_ty = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Scalar { kind, width },
            },
            Span::UNDEFINED,
        );

        self.types.insert(
            crate::Type {
                name: Some(format!(
                    "__atomic_compare_exchange_result<{kind:?},{width}>"
                )),
                inner: crate::TypeInner::Struct {
                    members: vec![
                        crate::StructMember {
                            name: Some("old_value".to_string()),
                            ty: scalar_ty,
                            binding: None,
                            offset: 0,
                        },
                        crate::StructMember {
                            name: Some("exchanged".to_string()),
                            ty: bool_ty,
                            binding: None,
                            offset: 4,
                        },
                    ],
                    span: 8,
                },
            },
            Span::UNDEFINED,
        )
    }
    /// Populate this module's [`SpecialTypes::ray_desc`] type.
    ///
    /// [`SpecialTypes::ray_desc`] is the type of the [`descriptor`] operand of
    /// an [`Initialize`] [`RayQuery`] statement. In WGSL, it is a struct type
    /// referred to as `RayDesc`.
    ///
    /// Backends consume values of this type to drive platform APIs, so if you
    /// change any its fields, you must update the backends to match. Look for
    /// backend code dealing with [`RayQueryFunction::Initialize`].
    ///
    /// [`SpecialTypes::ray_desc`]: crate::SpecialTypes::ray_desc
    /// [`descriptor`]: crate::RayQueryFunction::Initialize::descriptor
    /// [`Initialize`]: crate::RayQueryFunction::Initialize
    /// [`RayQuery`]: crate::Statement::RayQuery
    /// [`RayQueryFunction::Initialize`]: crate::RayQueryFunction::Initialize
    pub fn generate_ray_desc_type(&mut self) -> Handle<crate::Type> {
        if let Some(handle) = self.special_types.ray_desc {
            return handle;
        }

        let width = 4;
        let ty_flag = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Scalar {
                    width,
                    kind: crate::ScalarKind::Uint,
                },
            },
            Span::UNDEFINED,
        );
        let ty_scalar = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Scalar {
                    width,
                    kind: crate::ScalarKind::Float,
                },
            },
            Span::UNDEFINED,
        );
        let ty_vector = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Vector {
                    size: crate::VectorSize::Tri,
                    kind: crate::ScalarKind::Float,
                    width,
                },
            },
            Span::UNDEFINED,
        );

        let handle = self.types.insert(
            crate::Type {
                name: Some("RayDesc".to_string()),
                inner: crate::TypeInner::Struct {
                    members: vec![
                        crate::StructMember {
                            name: Some("flags".to_string()),
                            ty: ty_flag,
                            binding: None,
                            offset: 0,
                        },
                        crate::StructMember {
                            name: Some("cull_mask".to_string()),
                            ty: ty_flag,
                            binding: None,
                            offset: 4,
                        },
                        crate::StructMember {
                            name: Some("tmin".to_string()),
                            ty: ty_scalar,
                            binding: None,
                            offset: 8,
                        },
                        crate::StructMember {
                            name: Some("tmax".to_string()),
                            ty: ty_scalar,
                            binding: None,
                            offset: 12,
                        },
                        crate::StructMember {
                            name: Some("origin".to_string()),
                            ty: ty_vector,
                            binding: None,
                            offset: 16,
                        },
                        crate::StructMember {
                            name: Some("dir".to_string()),
                            ty: ty_vector,
                            binding: None,
                            offset: 32,
                        },
                    ],
                    span: 48,
                },
            },
            Span::UNDEFINED,
        );

        self.special_types.ray_desc = Some(handle);
        handle
    }

    /// Populate this module's [`SpecialTypes::ray_intersection`] type.
    ///
    /// [`SpecialTypes::ray_intersection`] is the type of a
    /// `RayQueryGetIntersection` expression. In WGSL, it is a struct type
    /// referred to as `RayIntersection`.
    ///
    /// Backends construct values of this type based on platform APIs, so if you
    /// change any its fields, you must update the backends to match. Look for
    /// the backend's handling for [`Expression::RayQueryGetIntersection`].
    ///
    /// [`SpecialTypes::ray_intersection`]: crate::SpecialTypes::ray_intersection
    /// [`Expression::RayQueryGetIntersection`]: crate::Expression::RayQueryGetIntersection
    pub fn generate_ray_intersection_type(&mut self) -> Handle<crate::Type> {
        if let Some(handle) = self.special_types.ray_intersection {
            return handle;
        }

        let width = 4;
        let ty_flag = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Scalar {
                    width,
                    kind: crate::ScalarKind::Uint,
                },
            },
            Span::UNDEFINED,
        );
        let ty_scalar = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Scalar {
                    width,
                    kind: crate::ScalarKind::Float,
                },
            },
            Span::UNDEFINED,
        );
        let ty_barycentrics = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Vector {
                    width,
                    size: crate::VectorSize::Bi,
                    kind: crate::ScalarKind::Float,
                },
            },
            Span::UNDEFINED,
        );
        let ty_bool = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Scalar {
                    width: crate::BOOL_WIDTH,
                    kind: crate::ScalarKind::Bool,
                },
            },
            Span::UNDEFINED,
        );
        let ty_transform = self.types.insert(
            crate::Type {
                name: None,
                inner: crate::TypeInner::Matrix {
                    columns: crate::VectorSize::Quad,
                    rows: crate::VectorSize::Tri,
                    width,
                },
            },
            Span::UNDEFINED,
        );

        let handle = self.types.insert(
            crate::Type {
                name: Some("RayIntersection".to_string()),
                inner: crate::TypeInner::Struct {
                    members: vec![
                        crate::StructMember {
                            name: Some("kind".to_string()),
                            ty: ty_flag,
                            binding: None,
                            offset: 0,
                        },
                        crate::StructMember {
                            name: Some("t".to_string()),
                            ty: ty_scalar,
                            binding: None,
                            offset: 4,
                        },
                        crate::StructMember {
                            name: Some("instance_custom_index".to_string()),
                            ty: ty_flag,
                            binding: None,
                            offset: 8,
                        },
                        crate::StructMember {
                            name: Some("instance_id".to_string()),
                            ty: ty_flag,
                            binding: None,
                            offset: 12,
                        },
                        crate::StructMember {
                            name: Some("sbt_record_offset".to_string()),
                            ty: ty_flag,
                            binding: None,
                            offset: 16,
                        },
                        crate::StructMember {
                            name: Some("geometry_index".to_string()),
                            ty: ty_flag,
                            binding: None,
                            offset: 20,
                        },
                        crate::StructMember {
                            name: Some("primitive_index".to_string()),
                            ty: ty_flag,
                            binding: None,
                            offset: 24,
                        },
                        crate::StructMember {
                            name: Some("barycentrics".to_string()),
                            ty: ty_barycentrics,
                            binding: None,
                            offset: 28,
                        },
                        crate::StructMember {
                            name: Some("front_face".to_string()),
                            ty: ty_bool,
                            binding: None,
                            offset: 36,
                        },
                        crate::StructMember {
                            name: Some("object_to_world".to_string()),
                            ty: ty_transform,
                            binding: None,
                            offset: 48,
                        },
                        crate::StructMember {
                            name: Some("world_to_object".to_string()),
                            ty: ty_transform,
                            binding: None,
                            offset: 112,
                        },
                    ],
                    span: 176,
                },
            },
            Span::UNDEFINED,
        );

        self.special_types.ray_intersection = Some(handle);
        handle
    }
}
