# [allow (non_snake_case)] # [allow (non_camel_case_types)] # [allow (clippy :: style)] # [allow (clippy :: complexity)] # [allow (unused_braces , unused_parens)] # [allow (clippy :: erasing_op)] # [allow (clippy :: approx_constant)] # [allow (clippy :: eq_op)] # [allow (clippy :: cmp_owned)] # [allow (clippy :: redundant_clone)] # [allow (clippy :: overly_complex_bool_expr)] mod slint_generatedAppWindow {
     use slint :: private_unstable_api :: re_exports as sp ;
     # [allow (unused_imports)] use sp :: {
         RepeatedItemTree as _ , ModelExt as _ , Model as _ , Float as _ }
     ;
     # [derive (Default , PartialEq , Debug , Clone)] pub struct r#TextStyle {
         pub r#font_size : f32 , pub r#font_weight : i32 }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerColorSchemeSelector_80 {
         r#dark_color_scheme : sp :: Property < bool > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , }
     impl InnerColorSchemeSelector_80 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) {
             # ! [allow (unused)] self . root . set (sp :: VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . dark_color_scheme ()) as _ }
                ) ;
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerCupertinoPalette_81 {
         root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , }
     impl InnerCupertinoPalette_81 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) {
             # ! [allow (unused)] self . root . set (sp :: VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerStyleMetrics_82 {
         root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , }
     impl InnerStyleMetrics_82 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) {
             # ! [allow (unused)] self . root . set (sp :: VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerLineEditBase_root_43 {
         r#root_43 : sp :: r#Empty , r#root_clip_44 : sp :: r#Clip , r#i_placeholder_45 : sp :: r#Text , r#i_text_input_46 : sp :: r#TextInput , r#root_43_has_focus : sp :: Property < bool > , r#root_43_height : sp :: Property < sp :: LogicalLength > , r#root_43_i_placeholder_45_horizontal_stretch : sp :: Property < f32 > , r#root_43_i_placeholder_45_max_height : sp :: Property < sp :: LogicalLength > , r#root_43_i_placeholder_45_max_width : sp :: Property < sp :: LogicalLength > , r#root_43_i_placeholder_45_min_height : sp :: Property < sp :: LogicalLength > , r#root_43_i_placeholder_45_min_width : sp :: Property < sp :: LogicalLength > , r#root_43_i_placeholder_45_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_43_i_placeholder_45_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_43_i_placeholder_45_vertical_stretch : sp :: Property < f32 > , r#root_43_i_placeholder_45_x : sp :: Property < sp :: LogicalLength > , r#root_43_i_placeholder_45_y : sp :: Property < sp :: LogicalLength > , r#root_43_i_text_input_46_computed_x : sp :: Property < sp :: LogicalLength > , r#root_43_i_text_input_46_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_43_i_text_input_46_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_43_i_text_input_46_x : sp :: Property < sp :: LogicalLength > , r#root_43_i_text_input_46_y : sp :: Property < sp :: LogicalLength > , r#root_43_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_43_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_43_margin : sp :: Property < sp :: LogicalLength > , r#root_43_placeholder_color : sp :: Property < slint :: Brush > , r#root_43_placeholder_text : sp :: Property < sp :: SharedString > , r#root_43_root_clip_44_x : sp :: Property < sp :: LogicalLength > , r#root_43_root_clip_44_y : sp :: Property < sp :: LogicalLength > , r#root_43_text_color : sp :: Property < slint :: Brush > , r#root_43_width : sp :: Property < sp :: LogicalLength > , r#root_43_x : sp :: Property < sp :: LogicalLength > , r#root_43_y : sp :: Property < sp :: LogicalLength > , r#root_43_accepted : sp :: Callback < (sp :: SharedString ,) , () > , r#root_43_edited : sp :: Callback < (sp :: SharedString ,) , () > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerLineEditBase_root_43 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerLineEditBase_root_43 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_has_focus }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_max_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_vertical_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_46_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_46_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_46_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min (((((((({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) - (1f64 as f64)) as sp :: Coord) . max ((({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_46_computed_x }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_max_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_min_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_max_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_min_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_preferred_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_vertical_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_clip_44 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_placeholder_color }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((((({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ()) == (sp :: SharedString :: from ("")))) && (((({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#preedit_text) . apply_pin (_self) . get ()) == (sp :: SharedString :: from (""))))) {
                         ({
                             * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_placeholder_text }
                        ) . apply_pin (_self) . get () }
                     else {
                         (sp :: SharedString :: from ("")) as _ }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#accepted) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_accepted }
                            ) . apply_pin (_self) . call (& (({
                                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_text_color }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#cursor_position_changed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#x as f64) + (({
                                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_46_computed_x }
                            ) . apply_pin (_self) . get () . get () as f64)) as f64) < (({
                                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_margin }
                            ) . apply_pin (_self) . get () . get () as f64)) {
                                 {
                                     ({
                                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_46_computed_x }
                                    ) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- (args . 0 . clone ()) . r#x as f64) + (({
                                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_margin }
                                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord) as _) }
                                 }
                             else {
                                 (if (((((args . 0 . clone ()) . r#x as f64) + (({
                                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_46_computed_x }
                                ) . apply_pin (_self) . get () . get () as f64)) as f64) > (((((({
                                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_width }
                                ) . apply_pin (_self) . get () . get () as f64) - (({
                                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_margin }
                                ) . apply_pin (_self) . get () . get () as f64)) as f64) - (1f64 as f64)) as f64)) {
                                     {
                                         ({
                                             * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_46_computed_x }
                                        ) . apply_pin (_self) . set (sp :: LogicalLength :: new (((((((({
                                             * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_width }
                                        ) . apply_pin (_self) . get () . get () as f64) - ((args . 0 . clone ()) . r#x as f64)) as f64) - (({
                                             * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_margin }
                                        ) . apply_pin (_self) . get () . get () as f64)) as f64) - (1f64 as f64)) as sp :: Coord) as _) }
                                     }
                                 else {
                                     ({
                                         }
                                    ) as _ }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#edited) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_edited }
                            ) . apply_pin (_self) . call (& (({
                                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (false) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Color :: from_argb_encoded (4286611584f64 as u32)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (1f64 as f64)) as sp :: Coord) . max ((({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_46_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_46_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_root_clip_44_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_root_clip_44_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_clip_44 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_clip_44 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_clip_44 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_clip_44 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_clip_44 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (50f64 as sp :: Coord) . max ((({
                             * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_min_width }
                        ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = ({
                             * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_46_preferred_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_root_clip_44_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_root_clip_44_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 2u32 => (((1f64 as f64) * (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_height }
                ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord , ((1f64 as f64) * (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_width }
                ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord , ({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 3u32 => (((1f64 as f64) * (({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_height }
                ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord , ({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_46_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_46_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 2u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (2u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , _ => :: core :: default :: Default :: default () , }
             }
         # [allow (dead_code , unused)] pub fn r#fn_clear_selection (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ({
                 ({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                ) . apply_pin (_self) . r#clear_selection ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) }
            ) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_copy (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ({
                 ({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                ) . apply_pin (_self) . r#copy ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) }
            ) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_cut (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ({
                 ({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                ) . apply_pin (_self) . r#cut ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) }
            ) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_focus (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1))) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_paste (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ({
                 ({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                ) . apply_pin (_self) . r#paste ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) }
            ) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_select_all (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ({
                 ({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                ) . apply_pin (_self) . r#select_all ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) }
            ) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_set_selection_offsets (self : :: core :: pin :: Pin < & Self > , arg_0 : i32 , arg_1 : i32 ,) -> () {
             let _self = self ;
             let args = (arg_0 , arg_1 ,) ;
             ({
                 ({
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                ) . apply_pin (_self) . set_selection_offsets ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1) , args . 0 . clone () as i32 , args . 1 . clone () as i32) }
            ) ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerLineEdit_root_47 {
         r#root_47 : sp :: r#Empty , r#_opacity_48 : sp :: r#Opacity , r#rectangle_49 : sp :: r#Rectangle , r#i_background_50 : sp :: r#BasicBorderRectangle , r#i_base_52 : InnerLineEditBase_root_43 , r#root_47__opacity_48_dummy : sp :: Property < sp :: LogicalLength > , r#root_47_height : sp :: Property < sp :: LogicalLength > , r#root_47_i_background_50_x : sp :: Property < sp :: LogicalLength > , r#root_47_i_background_50_y : sp :: Property < sp :: LogicalLength > , r#root_47_i_layout_51_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_47_i_layout_51_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_47_i_layout_51_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_47_rectangle_49_x : sp :: Property < sp :: LogicalLength > , r#root_47_rectangle_49_y : sp :: Property < sp :: LogicalLength > , r#root_47_state : sp :: Property < i32 > , r#root_47_width : sp :: Property < sp :: LogicalLength > , r#root_47_x : sp :: Property < sp :: LogicalLength > , r#root_47_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerLineEdit_root_47 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerLineEdit_root_47 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerLineEditBase_root_43 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#i_base_52 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 3u32 - 1 , tree_index_of_first_child + 5u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_layout_51_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
                                 + {
                                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (50f64 as sp :: Coord) . max ((({
                                         InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
                                     + {
                                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_min_width }
                                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 7f64 as _ ;
                             the_struct . r#end = 7f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_width }
                        ) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_layout_51_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
                             + {
                                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (50f64 as sp :: Coord) . max ((({
                                     InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
                                 + {
                                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_min_width }
                                ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 7f64 as _ ;
                         the_struct . r#end = 7f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_layout_51_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
                             + {
                                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ({
                                     InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
                                 + {
                                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_46_preferred_height }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_rectangle_49_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (((({
                         * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_width }
                    ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_rectangle_49_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_height }
                    ) . apply_pin (_self) . get () . get () as f64) - (((({
                         * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_height }
                    ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ! ({
                         InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
                     + {
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (if ({
                             InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
                         + {
                             * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_has_focus }
                        ) . apply_pin (_self) . get () {
                             2f64 }
                         else {
                             (0f64) as _ }
                        ) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#_opacity_48 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((if ({
                         InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
                     + {
                         * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_has_focus }
                    ) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (0f64) as _ }
                     as f64) == (1f64 as f64)) {
                         1f64 }
                     else {
                         (0f64) as _ }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#rectangle_49 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4282940159f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4284916195f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_background_50 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_47_state = ({
                             * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_root_47_state . clone () as f64) == (1f64 as f64)) {
                             slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                                 sp :: Color :: from_argb_encoded (3009503585f64 as u32) }
                             else {
                                 (sp :: Color :: from_argb_encoded (3019898879f64 as u32)) as _ }
                            ) }
                         else {
                             (if ((r#tmp_root_47_state . clone () as f64) == (2f64 as f64)) {
                                 slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (4284572001f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                                ) }
                             else {
                                 (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (4281084972f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (83886080f64 as u32)) as _ }
                                )) as _ }
                            ) as _ }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_background_50 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (654311423f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (637534208f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_background_50 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
                 + {
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((0.9996999999999999f64 as f64) * (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: WindowItem :: FIELD_OFFSETS . default_font_size) . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
             + {
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (400f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
                 + {
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
             + {
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_margin }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
                 + {
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_placeholder_color }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_state }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (1090519039f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (1073741824f64 as u32)) as _ }
                        ) }
                     else {
                         (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (1090519039f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (1073741824f64 as u32)) as _ }
                        )) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
                 + {
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (1291867601f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (1291877119f64 as u32)) as _ }
                    ) . color ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
                 + {
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                    ) . color ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
                 + {
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_text_color }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_state }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (1090519039f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (1073741824f64 as u32)) as _ }
                        ) }
                     else {
                         (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                        )) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
                 + {
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_layout_51_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
                 + {
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_layout_51_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47__opacity_48_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_background_50_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_background_50_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_background_50 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_background_50 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
             + {
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
             + {
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_placeholder_45_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
             + {
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_i_text_input_46_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
             + {
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_margin }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
             + {
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_root_clip_44_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
             + {
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_root_clip_44_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
             + {
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_y }
            ) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             InnerLineEditBase_root_43 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#i_base_52 }
             . apply_pin (x)) ,) ;
             {
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_layout_51_layoutinfo_h }
                    ) . apply_pin (_self) . get ())) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (160f64 as sp :: Coord) . max (((({
                             * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_layout_51_layoutinfo_h }
                        ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_layout_51_layoutinfo_v }
                    ) . apply_pin (_self) . get ())) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (22f64 as sp :: Coord) . max (((({
                             * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_layout_51_layoutinfo_v }
                        ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 0f64 as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (((({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_height }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ((({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_width }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_rectangle_49_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_rectangle_49_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 2u32 => (({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_background_50_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_background_50_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 3u32 => (({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_layout_51_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_layout_51_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , ({
                     InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
                 + {
                     * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 4u32 => (((({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_height }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ((({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_width }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47__opacity_48_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47__opacity_48_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 5u32 ..= 7u32 => return {
                     * & Self :: FIELD_OFFSETS . r#i_base_52 }
                 . apply_pin (_self) . item_geometry (index - 5u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 3u32 => {
                     * & Self :: FIELD_OFFSETS . r#i_base_52 }
                 . apply_pin (_self) . accessible_role (0) , 5u32 ..= 7u32 => {
                     * & Self :: FIELD_OFFSETS . r#i_base_52 }
                 . apply_pin (_self) . accessible_role (index - 5u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (3u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#i_base_52 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (5u32 ..= 7u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#i_base_52 }
                 . apply_pin (_self) . accessible_string_property (index - 5u32 + 1 , what) , _ => :: core :: default :: Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerButton_root_53 {
         r#root_53 : sp :: r#Empty , r#_opacity_54 : sp :: r#Opacity , r#rectangle_55 : sp :: r#BasicBorderRectangle , r#i_touch_area_78 : sp :: r#TouchArea , r#i_focus_scope_79 : sp :: r#FocusScope , r#root_53__opacity_54_dummy : sp :: Property < sp :: LogicalLength > , r#root_53_background : sp :: Property < slint :: Brush > , r#root_53_checkable : sp :: Property < bool > , r#root_53_checked : sp :: Property < bool > , r#root_53_height : sp :: Property < sp :: LogicalLength > , r#root_53_i_focus_scope_79_y : sp :: Property < sp :: LogicalLength > , r#root_53_i_layout_71_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_53_i_layout_71_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_53_i_layout_71_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_53_i_touch_area_78_x : sp :: Property < sp :: LogicalLength > , r#root_53_i_touch_area_78_y : sp :: Property < sp :: LogicalLength > , r#root_53_icon : sp :: Property < sp :: Image > , r#root_53_pressed : sp :: Property < bool > , r#root_53_rectangle_55_x : sp :: Property < sp :: LogicalLength > , r#root_53_rectangle_55_y : sp :: Property < sp :: LogicalLength > , r#root_53_state : sp :: Property < i32 > , r#root_53_text : sp :: Property < sp :: SharedString > , r#root_53_text_color : sp :: Property < slint :: Brush > , r#root_53_width : sp :: Property < sp :: LogicalLength > , r#root_53_x : sp :: Property < sp :: LogicalLength > , r#root_53_y : sp :: Property < sp :: LogicalLength > , r#root_53_clicked : sp :: Callback < () , () > , repeater0 : sp :: Repeater < InnerComponent__shadow_56 > , repeater1 : sp :: Repeater < InnerComponent__shadow_65 > , repeater2 : sp :: Repeater < InnerComponent__opacity_72 > , repeater3 : sp :: Repeater < InnerComponent__opacity_75 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerButton_root_53 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerButton_root_53 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (false as bool)) as _ }
                 }
            ) ;
             _self . repeater1 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (((! false) || (! ({
                         * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_focus_scope_79 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ())) as bool)) as _ }
                 }
            ) ;
             _self . repeater2 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new ({
                         let r#tmp_root_53_icon = sp :: Image :: default () ;
                         ;
                         (((((r#tmp_root_53_icon . clone () . size ()) . r#width as f64) > (0f64 as f64))) && ((((r#tmp_root_53_icon . clone () . size ()) . r#height as f64) > (0f64 as f64)))) }
                     as bool)) as _ }
                 }
            ) ;
             _self . repeater3 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (((({
                         * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text }
                    ) . apply_pin (_self) . get ()) != (sp :: SharedString :: from (""))) as bool)) as _ }
                 }
            ) ;
             sp :: Property :: link_two_way (({
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_touch_area_78 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , ({
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_focus_scope_79 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self)) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_background }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_53_state = ({
                             * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_root_53_state . clone () as f64) == (1f64 as f64)) {
                             slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                                 sp :: Color :: from_argb_encoded (2153865569f64 as u32) }
                             else {
                                 (sp :: Color :: from_argb_encoded (2164260863f64 as u32)) as _ }
                            ) }
                         else {
                             (if ((r#tmp_root_53_state . clone () as f64) == (2f64 as f64)) {
                                 if false {
                                     slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                                         sp :: Color :: from_argb_encoded (4280317678f64 as u32) }
                                     else {
                                         (sp :: Color :: from_argb_encoded (4278215658f64 as u32)) as _ }
                                    ) }
                                 else {
                                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                                         sp :: Color :: from_argb_encoded (4286216826f64 as u32) }
                                     else {
                                         (sp :: Color :: from_argb_encoded (4293980400f64 as u32)) as _ }
                                    )) as _ }
                                 }
                             else {
                                 (if false {
                                     slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                                         sp :: Color :: from_argb_encoded (4278212049f64 as u32) }
                                     else {
                                         (sp :: Color :: from_argb_encoded (4278221567f64 as u32)) as _ }
                                    ) }
                                 else {
                                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                                         sp :: Color :: from_argb_encoded (4284572001f64 as u32) }
                                     else {
                                         (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                                    )) as _ }
                                ) as _ }
                            ) as _ }
                         }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 2usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater2 . len () + _self . repeater3 . len ()) ;
                         InnerButton_root_53 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent__opacity_72 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater2 . instances_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         InnerButton_root_53 :: FIELD_OFFSETS . repeater3 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent__opacity_75 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater3 . instances_vec () ;
                         r#repeated_indices [1usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [1usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = 8f64 as _ ;
                                 the_struct . r#end = 8f64 as _ ;
                                 the_struct }
                             as _ , r#size : ({
                                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width }
                            ) . apply_pin (_self) . get () . get () as _ , r#spacing : 4f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater2 . len () + _self . repeater3 . len ()) ;
                         InnerButton_root_53 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent__opacity_72 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater2 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         InnerButton_root_53 :: FIELD_OFFSETS . repeater3 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent__opacity_75 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater3 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells . clone () as _ , 4f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Center as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater2 . len () + _self . repeater3 . len ()) ;
                         InnerButton_root_53 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent__opacity_72 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater2 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         InnerButton_root_53 :: FIELD_OFFSETS . repeater3 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent__opacity_75 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater3 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells . clone () as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 4f64 as _ ;
                             the_struct . r#end = 4f64 as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_pressed }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_focus_scope_79 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ()) && (({
                         * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_touch_area_78 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_rectangle_55_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (((({
                         * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width }
                    ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_rectangle_55_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height }
                    ) . apply_pin (_self) . get () . get () as f64) - (((({
                         * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height }
                    ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ! ({
                         * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_focus_scope_79 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (if ({
                             * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_pressed }
                        ) . apply_pin (_self) . get () {
                             2f64 }
                         else {
                             (if ({
                                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_checked }
                            ) . apply_pin (_self) . get () {
                                 3f64 }
                             else {
                                 (0f64) as _ }
                            ) as _ }
                        ) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text_color }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_53_state = ({
                             * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_root_53_state . clone () as f64) == (1f64 as f64)) {
                             slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                                 sp :: Color :: from_argb_encoded (1090519039f64 as u32) }
                             else {
                                 (sp :: Color :: from_argb_encoded (1073741824f64 as u32)) as _ }
                            ) }
                         else {
                             (if ((r#tmp_root_53_state . clone () as f64) == (3f64 as f64)) {
                                 slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (4280317678f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (4278215658f64 as u32)) as _ }
                                ) }
                             else {
                                 (if false {
                                     slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4293980400f64 as u32)) }
                                 else {
                                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                                     else {
                                         (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                                    )) as _ }
                                ) as _ }
                            ) as _ }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#_opacity_54 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((if ({
                         * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_focus_scope_79 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (0f64) as _ }
                     as f64) == (1f64 as f64)) {
                         1f64 }
                     else {
                         (0f64) as _ }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#rectangle_55 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4282940159f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4284916195f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#rectangle_55 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_touch_area_78 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ({
                                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_checkable }
                            ) . apply_pin (_self) . get () {
                                 {
                                     ({
                                         * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_checked }
                                    ) . apply_pin (_self) . set (! ({
                                         * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_checked }
                                    ) . apply_pin (_self) . get () as _) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             ;
                             ({
                                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_clicked }
                            ) . apply_pin (_self) . call (& () . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_focus_scope_79 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_focus_scope_79 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#key_pressed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let r#returned_expression0 = {
                                 let r#return_check_merge0 = if ! (((((args . 0 . clone ()) . r#text) == (sp :: SharedString :: from (" ")))) || ((((args . 0 . clone ()) . r#text) == (sp :: SharedString :: from ("\n"))))) {
                                     (true , sp :: r#EventResult :: r#Reject ,) }
                                 else {
                                     ((false , {
                                         ({
                                             * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_touch_area_78 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) . call (& () . into ()) ;
                                         sp :: r#EventResult :: r#Accept }
                                     ,)) as _ }
                                 ;
                                 ;
                                 if (r#return_check_merge0 . clone ()) . 0 {
                                     (false , {
                                         sp :: r#EventResult :: r#Reject }
                                     ,) }
                                 else {
                                     ((false , (r#return_check_merge0 . clone ()) . 1 ,)) as _ }
                                 }
                             ;
                             ;
                             if (r#returned_expression0 . clone ()) . 0 {
                                 sp :: r#EventResult :: r#Reject }
                             else {
                                 ((r#returned_expression0 . clone ()) . 1) as _ }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             ({
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53__opacity_54_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_checkable }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_focus_scope_79_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_touch_area_78_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_touch_area_78_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#rectangle_55 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#rectangle_55 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#rectangle_55 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_touch_area_78 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             {
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_53 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent__shadow_56 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1u32 => {
                     InnerButton_root_53 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent__shadow_65 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater1 . visit (order , visitor) }
                 2u32 => {
                     InnerButton_root_53 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent__opacity_72 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater2 . visit (order , visitor) }
                 3u32 => {
                     InnerButton_root_53 :: FIELD_OFFSETS . repeater3 . apply_pin (_self) . ensure_updated (|| InnerComponent__opacity_75 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater3 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                    ) . apply_pin (_self) . get ())) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (20f64 as sp :: Coord) . max (((({
                             * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                        ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 0f64 as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_v }
                    ) . apply_pin (_self) . get ())) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (20f64 as sp :: Coord) . max (((({
                             * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_v }
                        ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 0f64 as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_53 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent__shadow_56 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 => {
                     InnerButton_root_53 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent__shadow_65 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater1 . range ()) }
                 2u32 => {
                     InnerButton_root_53 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent__opacity_72 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater2 . range ()) }
                 3u32 => {
                     InnerButton_root_53 :: FIELD_OFFSETS . repeater3 . apply_pin (_self) . ensure_updated (|| InnerComponent__opacity_75 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater3 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_53 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent__shadow_56 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 => {
                     InnerButton_root_53 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent__shadow_65 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater1 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 2u32 => {
                     InnerButton_root_53 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent__opacity_72 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater2 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 3u32 => {
                     InnerButton_root_53 :: FIELD_OFFSETS . repeater3 . apply_pin (_self) . ensure_updated (|| InnerComponent__opacity_75 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater3 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (((({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ((({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_rectangle_55_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_rectangle_55_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 6u32 => (({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_touch_area_78_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_touch_area_78_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 7u32 => (({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord , ({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_focus_scope_79_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 8u32 => (((({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ((({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53__opacity_54_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53__opacity_54_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Button , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text }
                ) . apply_pin (_self) . get () , _ => :: core :: default :: Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent__shadow_56 {
         r#_shadow_56 : sp :: r#BoxShadow , r#rectangle_57 : sp :: r#BasicBorderRectangle , r#_shadow_58 : sp :: r#BoxShadow , r#rectangle_59 : sp :: r#BasicBorderRectangle , r#_shadow_60 : sp :: r#BoxShadow , r#rectangle_61 : sp :: r#BasicBorderRectangle , r#_opacity_62 : sp :: r#Opacity , r#rectangle_63 : sp :: r#BasicBorderRectangle , r#_shadow_56__opacity_62_dummy : sp :: Property < sp :: LogicalLength > , r#_shadow_56_dummy : sp :: Property < sp :: LogicalLength > , r#_shadow_56_rectangle_57_x : sp :: Property < sp :: LogicalLength > , r#_shadow_56_rectangle_57_y : sp :: Property < sp :: LogicalLength > , r#_shadow_56_rectangle_59_x : sp :: Property < sp :: LogicalLength > , r#_shadow_56_rectangle_59_y : sp :: Property < sp :: LogicalLength > , r#_shadow_56_rectangle_61_x : sp :: Property < sp :: LogicalLength > , r#_shadow_56_rectangle_61_y : sp :: Property < sp :: LogicalLength > , r#_shadow_56_rectangle_63_x : sp :: Property < sp :: LogicalLength > , r#_shadow_56_rectangle_63_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent__shadow_56 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_53 > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent__shadow_56 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (3f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded (1711276032f64 as u32)) as sp :: Color }
            ) ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0.5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_57 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_background) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_57 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_58 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_58 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_58 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded (637534208f64 as u32)) as sp :: Color }
            ) ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_58 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_59 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_background) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_59 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_60 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_60 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_60 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded (637534208f64 as u32)) as sp :: Color }
            ) ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_60 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0.5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_61 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_background) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_61 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_opacity_62 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (0.17f64) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_63 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                     color : sp :: Color :: from_argb_encoded (4294967295f64 as u32) , position : 1f64 as _ }
                 , sp :: GradientStop {
                     color : sp :: Color :: from_argb_encoded (16777215f64 as u32) , position : 0f64 as _ }
                ]))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_63 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56__opacity_62_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56_rectangle_57_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56_rectangle_57_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56_rectangle_59_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56_rectangle_59_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56_rectangle_61_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56_rectangle_61_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56_rectangle_63_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56_rectangle_63_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_57 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_57 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_57 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_58 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_58 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_58 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_58 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_58 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_59 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_59 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_59 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_60 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_60 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_60 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_60 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_60 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_61 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_61 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_61 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_63 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_63 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_63 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_63 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                 , sp :: Orientation :: Vertical => {
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56_rectangle_57_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56_rectangle_57_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => ((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 2u32 => ((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56_rectangle_59_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56_rectangle_59_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 3u32 => ((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56_rectangle_59_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56_rectangle_59_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 4u32 => ((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56_rectangle_61_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56_rectangle_61_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 5u32 => ((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56_rectangle_61_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56_rectangle_61_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 6u32 => ((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56_rectangle_63_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56_rectangle_63_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 7u32 => ((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56__opacity_62_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56__opacity_62_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => :: core :: default :: Default :: default () , }
             }
         }
     impl InnerComponent__shadow_56 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_53 >) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_53 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_item_tree (& self_dyn_rc , (* & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap ()) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             8usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 5u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 7u32 , parent_index : 1u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 7u32 , parent_index : 1u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 7u32 , parent_index : 1u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 7u32 , parent_index : 1u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 7u32 , parent_index : 1u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 8u32 , parent_index : 6u32 , item_array_index : 7u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent__shadow_56 , sp :: ItemVTable , sp :: AllowPin > ;
             8usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_56 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_57 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_58 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_59 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_shadow_60 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_61 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#_opacity_62 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_56 :: FIELD_OFFSETS . r#rectangle_63 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent__shadow_56) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent__shadow_56 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent__shadow_56 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent__shadow_56 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent__shadow_56 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 2u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent__shadow_56 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent__shadow_56 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent__shadow_65 {
         r#_shadow_65 : sp :: r#BoxShadow , r#rectangle_66 : sp :: r#BasicBorderRectangle , r#_opacity_67 : sp :: r#Opacity , r#_shadow_68 : sp :: r#BoxShadow , r#rectangle_69 : sp :: r#BasicBorderRectangle , r#_shadow_65__opacity_67_dummy : sp :: Property < sp :: LogicalLength > , r#_shadow_65_dummy : sp :: Property < sp :: LogicalLength > , r#_shadow_65_rectangle_66_x : sp :: Property < sp :: LogicalLength > , r#_shadow_65_rectangle_66_y : sp :: Property < sp :: LogicalLength > , r#_shadow_65_rectangle_69_x : sp :: Property < sp :: LogicalLength > , r#_shadow_65_rectangle_69_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent__shadow_65 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_53 > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent__shadow_65 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0.25f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded (1711276032f64 as u32)) as sp :: Color }
            ) ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0.25f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#rectangle_66 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_background) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#rectangle_66 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_opacity_67 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_focus_scope_79 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () {
                         1f64 }
                     else {
                         (0.5f64) as _ }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_68 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_68 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_68 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded (637534208f64 as u32)) as sp :: Color }
            ) ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_68 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#rectangle_69 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_background) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#rectangle_69 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (352321535f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (335544320f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#rectangle_69 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#rectangle_69 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65__opacity_67_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65_rectangle_66_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65_rectangle_66_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65_rectangle_69_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65_rectangle_69_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#rectangle_66 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#rectangle_66 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#rectangle_66 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_68 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_68 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_68 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_68 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_68 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#rectangle_69 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#rectangle_69 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                 , sp :: Orientation :: Vertical => {
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65_rectangle_66_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65_rectangle_66_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => ((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 2u32 => ((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65_rectangle_69_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65_rectangle_69_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 3u32 => ((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65__opacity_67_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65__opacity_67_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 4u32 => ((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65__opacity_67_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65__opacity_67_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => :: core :: default :: Default :: default () , }
             }
         }
     impl InnerComponent__shadow_65 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_53 >) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_53 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_item_tree (& self_dyn_rc , (* & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap ()) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             5usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 3u32 , parent_index : 1u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 5u32 , parent_index : 2u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 5u32 , parent_index : 2u32 , item_array_index : 4u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent__shadow_65 , sp :: ItemVTable , sp :: AllowPin > ;
             5usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_65 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#rectangle_66 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_opacity_67 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#_shadow_68 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_65 :: FIELD_OFFSETS . r#rectangle_69 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent__shadow_65) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent__shadow_65 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent__shadow_65 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent__shadow_65 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent__shadow_65 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 3u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent__shadow_65 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent__shadow_65 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent__opacity_72 {
         r#_opacity_72 : sp :: r#Opacity , r#image_73 : sp :: r#ImageItem , r#_opacity_72_dummy : sp :: Property < sp :: LogicalLength > , r#_opacity_72_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#_opacity_72_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent__opacity_72 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_53 > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent__opacity_72 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             sp :: Property :: link_two_way (({
                 * & InnerComponent__opacity_72 :: FIELD_OFFSETS . r#image_73 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) , (InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_icon) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ())) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_72 :: FIELD_OFFSETS . r#_opacity_72_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Item :: layout_info (({
                         * & InnerComponent__opacity_72 :: FIELD_OFFSETS . r#image_73 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_72 :: FIELD_OFFSETS . r#_opacity_72_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Item :: layout_info (({
                         * & InnerComponent__opacity_72 :: FIELD_OFFSETS . r#image_73 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_72 :: FIELD_OFFSETS . r#_opacity_72 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_focus_scope_79 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () {
                         1f64 }
                     else {
                         (0.5f64) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_72 :: FIELD_OFFSETS . r#image_73 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if false {
                         (InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text_color) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () }
                     else {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_72 :: FIELD_OFFSETS . r#image_73 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (4f64 as f64)) as f64) - (4f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__opacity_72 :: FIELD_OFFSETS . r#image_73 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             ({
                 * & InnerComponent__opacity_72 :: FIELD_OFFSETS . r#image_73 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (13f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__opacity_72 :: FIELD_OFFSETS . r#_opacity_72_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_72 :: FIELD_OFFSETS . r#image_73 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_72 :: FIELD_OFFSETS . r#image_73 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_72 :: FIELD_OFFSETS . r#image_73 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_72 :: FIELD_OFFSETS . r#image_73 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerComponent__opacity_72 :: FIELD_OFFSETS . r#_opacity_72_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 13f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 13f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => ({
                     * & InnerComponent__opacity_72 :: FIELD_OFFSETS . r#_opacity_72_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (4f64 as f64)) as f64) - (4f64 as f64)) as sp :: Coord , 13f64 as sp :: Coord , {
                     let cache = (InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                     * cache . get ((cache [0usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                 as sp :: Coord , 4f64 as sp :: Coord ,) , 1u32 => ((((((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (4f64 as f64)) as f64) - (4f64 as f64)) as sp :: Coord , 13f64 as sp :: Coord , ({
                     * & InnerComponent__opacity_72 :: FIELD_OFFSETS . r#_opacity_72_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__opacity_72 :: FIELD_OFFSETS . r#_opacity_72_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => :: core :: default :: Default :: default () , }
             }
         }
     impl InnerComponent__opacity_72 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_53 >) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_53 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_item_tree (& self_dyn_rc , (* & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap ()) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             2usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent__opacity_72 , sp :: ItemVTable , sp :: AllowPin > ;
             2usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerComponent__opacity_72 :: FIELD_OFFSETS . r#_opacity_72 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__opacity_72 :: FIELD_OFFSETS . r#image_73 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent__opacity_72) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent__opacity_72 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent__opacity_72 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent__opacity_72 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent__opacity_72 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 4u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent__opacity_72 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent__opacity_72 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent__opacity_75 {
         r#_opacity_75 : sp :: r#Opacity , r#text_76 : sp :: r#Text , r#_opacity_75_dummy : sp :: Property < sp :: LogicalLength > , r#_opacity_75_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#_opacity_75_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent__opacity_75 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_53 > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent__opacity_75 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#_opacity_75_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Item :: layout_info (({
                         * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#text_76 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#_opacity_75_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Item :: layout_info (({
                         * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#text_76 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#_opacity_75 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_focus_scope_79 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () {
                         1f64 }
                     else {
                         (0.5f64) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#text_76 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text_color) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get ()) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#text_76 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((0.9996999999999999f64 as f64) * (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: WindowItem :: FIELD_OFFSETS . default_font_size) . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#text_76 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (400f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#text_76 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (4f64 as f64)) as f64) - (4f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#text_76 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#text_76 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#text_76 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#text_76 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let cache = (InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                         * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#_opacity_75_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#text_76 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#text_76 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#text_76 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#text_76 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#text_76 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#text_76 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#text_76 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#text_76 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => ({
                     * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#_opacity_75_layoutinfo_h }
                ) . apply_pin (_self) . get () , sp :: Orientation :: Vertical => ({
                     * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#_opacity_75_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (4f64 as f64)) as f64) - (4f64 as f64)) as sp :: Coord , {
                     let cache = (InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                     * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                 as sp :: Coord , {
                     let cache = (InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                     * cache . get ((cache [2usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                 as sp :: Coord , 4f64 as sp :: Coord ,) , 1u32 => ((((((InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (4f64 as f64)) as f64) - (4f64 as f64)) as sp :: Coord , {
                     let cache = (InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                     * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                 as sp :: Coord , ({
                     * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#_opacity_75_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#_opacity_75_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (1u32 , sp :: AccessibleStringProperty :: r#Label) => (InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () , _ => :: core :: default :: Default :: default () , }
             }
         }
     impl InnerComponent__opacity_75 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_53 >) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_53 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_item_tree (& self_dyn_rc , (* & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap ()) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             2usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent__opacity_75 , sp :: ItemVTable , sp :: AllowPin > ;
             2usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#_opacity_75 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__opacity_75 :: FIELD_OFFSETS . r#text_76 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent__opacity_75) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent__opacity_75 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent__opacity_75 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent__opacity_75 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent__opacity_75 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 5u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent__opacity_75 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent__opacity_75 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerAppWindow {
         r#root_1 : sp :: r#WindowItem , r#empty_3 : sp :: r#Empty , r#text_4 : sp :: r#Text , r#empty_5 : sp :: r#Empty , r#text_6 : sp :: r#Text , r#text_8 : sp :: r#Text , r#empty_9 : sp :: r#Empty , r#empty_12 : sp :: r#Empty , r#empty_13 : sp :: r#Empty , r#i_flickable_14 : sp :: r#Flickable , r#i_flickable_viewport_15 : sp :: r#Empty , r#i_vertical_bar_visibility_19 : sp :: r#Clip , r#i_vertical_bar_20 : sp :: r#Rectangle , r#i_border_21 : sp :: r#Rectangle , r#i_thumb_opacity_22 : sp :: r#Opacity , r#i_thumb_23 : sp :: r#BasicBorderRectangle , r#i_touch_area_24 : sp :: r#TouchArea , r#i_horizontal_bar_visibility_25 : sp :: r#Clip , r#i_horizontal_bar_26 : sp :: r#Rectangle , r#i_border_27 : sp :: r#Rectangle , r#i_thumb_opacity_28 : sp :: r#Opacity , r#i_thumb_29 : sp :: r#BasicBorderRectangle , r#i_touch_area_30 : sp :: r#TouchArea , r#empty_31 : sp :: r#Empty , r#empty_35 : sp :: r#Empty , r#text_36 : sp :: r#Text , r#empty_38 : sp :: r#Empty , r#text_39 : sp :: r#Text , r#empty_40 : sp :: r#Empty , r#text_41 : sp :: r#Text , r#api_string_7 : InnerLineEdit_root_47 , r#button_10 : InnerButton_root_53 , r#button_11 : InnerButton_root_53 , r#button_32 : InnerButton_root_53 , r#button_33 : InnerButton_root_53 , r#button_34 : InnerButton_root_53 , r#db_string_37 : InnerLineEdit_root_47 , r#button_42 : InnerButton_root_53 , r#root_1_api_input : sp :: Property < sp :: SharedString > , r#root_1_db_input : sp :: Property < sp :: SharedString > , r#root_1_empty_12_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_12_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_12_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_13_visible_height : sp :: Property < sp :: LogicalLength > , r#root_1_empty_13_visible_width : sp :: Property < sp :: LogicalLength > , r#root_1_empty_13_y : sp :: Property < sp :: LogicalLength > , r#root_1_empty_2_layout_cache_h : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_2_layout_cache_v : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_2_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_2_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_3_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_3_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_3_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_31_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_31_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_31_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_35_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_35_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_35_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_38_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_38_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_38_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_40_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_40_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_40_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_5_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_5_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_5_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_9_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_9_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_9_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_error_output : sp :: Property < sp :: SharedString > , r#root_1_i_horizontal_bar_26_height : sp :: Property < sp :: LogicalLength > , r#root_1_i_horizontal_bar_26_maximum : sp :: Property < sp :: LogicalLength > , r#root_1_i_horizontal_bar_26_pad : sp :: Property < sp :: LogicalLength > , r#root_1_i_horizontal_bar_26_track_size : sp :: Property < sp :: LogicalLength > , r#root_1_i_horizontal_bar_26_visible : sp :: Property < bool > , r#root_1_i_horizontal_bar_26_width : sp :: Property < sp :: LogicalLength > , r#root_1_i_horizontal_bar_26_y : sp :: Property < sp :: LogicalLength > , r#root_1_i_horizontal_bar_visibility_25_height : sp :: Property < sp :: LogicalLength > , r#root_1_i_horizontal_bar_visibility_25_width : sp :: Property < sp :: LogicalLength > , r#root_1_i_horizontal_bar_visibility_25_x : sp :: Property < sp :: LogicalLength > , r#root_1_i_horizontal_bar_visibility_25_y : sp :: Property < sp :: LogicalLength > , r#root_1_i_thumb_23_height : sp :: Property < sp :: LogicalLength > , r#root_1_i_thumb_23_width : sp :: Property < sp :: LogicalLength > , r#root_1_i_thumb_23_x : sp :: Property < sp :: LogicalLength > , r#root_1_i_thumb_23_y : sp :: Property < sp :: LogicalLength > , r#root_1_i_thumb_29_height : sp :: Property < sp :: LogicalLength > , r#root_1_i_thumb_29_width : sp :: Property < sp :: LogicalLength > , r#root_1_i_thumb_29_x : sp :: Property < sp :: LogicalLength > , r#root_1_i_thumb_29_y : sp :: Property < sp :: LogicalLength > , r#root_1_i_thumb_opacity_22_dummy : sp :: Property < sp :: LogicalLength > , r#root_1_i_thumb_opacity_28_dummy : sp :: Property < sp :: LogicalLength > , r#root_1_i_touch_area_24_pressed_value : sp :: Property < sp :: LogicalLength > , r#root_1_i_touch_area_24_x : sp :: Property < sp :: LogicalLength > , r#root_1_i_touch_area_24_y : sp :: Property < sp :: LogicalLength > , r#root_1_i_touch_area_30_pressed_value : sp :: Property < sp :: LogicalLength > , r#root_1_i_touch_area_30_x : sp :: Property < sp :: LogicalLength > , r#root_1_i_touch_area_30_y : sp :: Property < sp :: LogicalLength > , r#root_1_i_vertical_bar_20_height : sp :: Property < sp :: LogicalLength > , r#root_1_i_vertical_bar_20_maximum : sp :: Property < sp :: LogicalLength > , r#root_1_i_vertical_bar_20_pad : sp :: Property < sp :: LogicalLength > , r#root_1_i_vertical_bar_20_track_size : sp :: Property < sp :: LogicalLength > , r#root_1_i_vertical_bar_20_visible : sp :: Property < bool > , r#root_1_i_vertical_bar_20_width : sp :: Property < sp :: LogicalLength > , r#root_1_i_vertical_bar_20_x : sp :: Property < sp :: LogicalLength > , r#root_1_i_vertical_bar_visibility_19_height : sp :: Property < sp :: LogicalLength > , r#root_1_i_vertical_bar_visibility_19_width : sp :: Property < sp :: LogicalLength > , r#root_1_i_vertical_bar_visibility_19_x : sp :: Property < sp :: LogicalLength > , r#root_1_i_vertical_bar_visibility_19_y : sp :: Property < sp :: LogicalLength > , r#root_1_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_preview_key : sp :: Property < sp :: ModelRc < i32 > > , r#root_1_preview_output : sp :: Property < sp :: SharedString > , r#root_1_response_output : sp :: Property < sp :: SharedString > , r#root_1_status_code_output : sp :: Property < sp :: SharedString > , r#root_1_x : sp :: Property < sp :: LogicalLength > , r#root_1_y : sp :: Property < sp :: LogicalLength > , r#root_1_api_response : sp :: Callback < (sp :: SharedString ,) , () > , r#root_1_api_response_dl : sp :: Callback < (sp :: SharedString ,) , () > , repeater0 : sp :: Repeater < InnerComponent_rectangle_16 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerAppWindow >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , globals : Globals_AppWindow , window_adapter_ : sp :: OnceCell < sp :: WindowAdapterRc > , }
     impl InnerAppWindow {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_preview_key }
                    ) . apply_pin (_self) . get ()) as _ }
                 }
            ) ;
             InnerLineEdit_root_47 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#api_string_7 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 12u32 - 1 , tree_index_of_first_child + 14u32 - 1) ;
             InnerButton_root_53 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_10 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 21u32 - 1 , tree_index_of_first_child + 23u32 - 1) ;
             InnerButton_root_53 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_11 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 22u32 - 1 , tree_index_of_first_child + 31u32 - 1) ;
             InnerButton_root_53 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_32 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 55u32 - 1 , tree_index_of_first_child + 58u32 - 1) ;
             InnerButton_root_53 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_33 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 56u32 - 1 , tree_index_of_first_child + 66u32 - 1) ;
             InnerButton_root_53 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_34 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 57u32 - 1 , tree_index_of_first_child + 74u32 - 1) ;
             InnerLineEdit_root_47 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#db_string_37 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 83u32 - 1 , tree_index_of_first_child + 84u32 - 1) ;
             InnerButton_root_53 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_42 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 9u32 - 1 , tree_index_of_first_child + 93u32 - 1) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_api_input }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4280821800f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                    ) . color ())) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_db_input }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_12_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = sp :: Item :: layout_info (({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#empty_13 }
                                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 50f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                        ) . apply_pin (_self) . get () [7usize] as _ , r#spacing : 0f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_12_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_13 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 50f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_12_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_13 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 50f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_visible_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                    ) . apply_pin (_self) . get () [7usize] as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_visible_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_12_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_grid_layout (& sp :: GridLayoutData {
                         r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_9_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_12_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_31_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_35_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_38_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_40_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
                                 + {
                                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (20f64 as sp :: Coord) . max (((({
                                         InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
                                     + {
                                         * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 25f64 as _ ;
                             the_struct . r#end = 25f64 as _ ;
                             the_struct }
                         as _ , r#size : 500f64 as _ , r#spacing : 15f64 as _ , }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_grid_layout (& sp :: GridLayoutData {
                         r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 1f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 2f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_9_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 3f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_12_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 4f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_31_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 5f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_35_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 6f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_38_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 7f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_40_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 8f64 as _ ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
                                 + {
                                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_v }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 30f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 30f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 25f64 as _ ;
                             the_struct . r#end = 25f64 as _ ;
                             the_struct }
                         as _ , r#size : 750f64 as _ , r#spacing : 15f64 as _ , }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#grid_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_9_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_12_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_31_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_35_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_38_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_40_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
                             + {
                                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (20f64 as sp :: Coord) . max (((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
                                 + {
                                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                    ]) as _ , 15f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 25f64 as _ ;
                         the_struct . r#end = 25f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#grid_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 1f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 2f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_9_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 3f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_12_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 4f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_31_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 5f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_35_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 6f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_38_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 7f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_40_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 8f64 as _ ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
                             + {
                                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 30f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 30f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                    ]) as _ , 15f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 25f64 as _ ;
                         the_struct . r#end = 25f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = sp :: Item :: layout_info (({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
                                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 75f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 75f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                        ) . apply_pin (_self) . get () [1usize] as _ , r#spacing : 10f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 75f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 75f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_31_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
                                 + {
                                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (20f64 as sp :: Coord) . max (((({
                                         InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
                                     + {
                                         * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
                                 + {
                                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (20f64 as sp :: Coord) . max (((({
                                         InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
                                     + {
                                         * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
                                 + {
                                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (20f64 as sp :: Coord) . max (((({
                                         InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
                                     + {
                                         * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                        ) . apply_pin (_self) . get () [9usize] as _ , r#spacing : 10f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_31_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
                             + {
                                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (20f64 as sp :: Coord) . max (((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
                                 + {
                                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
                             + {
                                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (20f64 as sp :: Coord) . max (((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
                                 + {
                                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
                             + {
                                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (20f64 as sp :: Coord) . max (((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
                                 + {
                                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_31_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
                             + {
                                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 30f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 30f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
                             + {
                                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 30f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 30f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
                             + {
                                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 30f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 30f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_35_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_36 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     InnerAppWindow :: FIELD_OFFSETS . r#db_string_37 }
                                 + {
                                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_layout_51_layoutinfo_h }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 300f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 300f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                        ) . apply_pin (_self) . get () [11usize] as _ , r#spacing : 10f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_35_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_36 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#db_string_37 }
                             + {
                                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_layout_51_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 300f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 300f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_35_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_36 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#db_string_37 }
                             + {
                                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_layout_51_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 20f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 20f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_38_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                        ) . apply_pin (_self) . get () [13usize] as _ , r#spacing : 10f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_38_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_38_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_40_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_41 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                        ) . apply_pin (_self) . get () [15usize] as _ , r#spacing : 10f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_40_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_41 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_40_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_41 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
                                 + {
                                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_layout_51_layoutinfo_h }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 300f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 300f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = sp :: Item :: layout_info (({
                                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_8 }
                                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 75f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 75f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                        ) . apply_pin (_self) . get () [3usize] as _ , r#spacing : 10f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
                             + {
                                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_layout_51_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 300f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 300f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_8 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 75f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 75f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
                             + {
                                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_layout_51_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 20f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 20f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_8 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_9_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
                                 + {
                                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (20f64 as sp :: Coord) . max (((({
                                         InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
                                     + {
                                         * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
                                 + {
                                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (20f64 as sp :: Coord) . max (((({
                                         InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
                                     + {
                                         * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                        ) . apply_pin (_self) . get () [5usize] as _ , r#spacing : 10f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_9_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
                             + {
                                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (20f64 as sp :: Coord) . max (((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
                                 + {
                                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
                             + {
                                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (20f64 as sp :: Coord) . max (((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
                                 + {
                                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_9_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
                             + {
                                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 30f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 30f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
                             + {
                                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_layout_71_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 30f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 30f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_error_output }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (750f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_30 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         20f64 }
                     else {
                         (12f64) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_maximum }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_visible_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_pad }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_30 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (0f64) as _ }
                     as f64) == (1f64 as f64)) {
                         4f64 }
                     else {
                         (2f64) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_track_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if true {
                         ((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64)) }
                     else {
                         (((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_visible }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64) > (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_visible_width }
                    ) . apply_pin (_self) . get () . get () as f64))) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_visible }
                    ) . apply_pin (_self) . get () {
                         ((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_12_layout_cache }
                        ) . apply_pin (_self) . get () [1usize] as f64) - (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_width }
                        ) . apply_pin (_self) . get () . get () as f64)) }
                     else {
                         (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_12_layout_cache }
                        ) . apply_pin (_self) . get () [1usize]) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                    ) . apply_pin (_self) . get () [7usize] as f64) - (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_23_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if false {
                         ((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (((2f64 as f64) * (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_pad }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) }
                     else {
                         ({
                             let r#tmp_i_vertical_bar_20_maximum = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_maximum }
                            ) . apply_pin (_self) . get () . get () ;
                             ;
                             ((if ((r#tmp_i_vertical_bar_20_maximum . clone () as f64) <= (((0f64 as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) as f64)) {
                                 0f64 }
                             else {
                                 ({
                                     let r#tmp_i_vertical_bar_20_page_size = ({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_visible_height }
                                    ) . apply_pin (_self) . get () . get () ;
                                     ;
                                     (((32f64 as sp :: Coord) . max ((((({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_track_size }
                                    ) . apply_pin (_self) . get () . get () as f64) * (((r#tmp_i_vertical_bar_20_page_size . clone () as f64) / (((r#tmp_i_vertical_bar_20_maximum . clone () as f64) + (r#tmp_i_vertical_bar_20_page_size . clone () as f64)) as f64)) as f64)) as sp :: Coord)) as f64) * (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                                ) as _ }
                             as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                        ) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_23_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ! false {
                         ((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (((2f64 as f64) * (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_pad }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) }
                     else {
                         ({
                             let r#tmp_i_vertical_bar_20_maximum = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_maximum }
                            ) . apply_pin (_self) . get () . get () ;
                             ;
                             ((if ((r#tmp_i_vertical_bar_20_maximum . clone () as f64) <= (((0f64 as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) as f64)) {
                                 0f64 }
                             else {
                                 ({
                                     let r#tmp_i_vertical_bar_20_page_size = ({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_visible_height }
                                    ) . apply_pin (_self) . get () . get () ;
                                     ;
                                     (((32f64 as sp :: Coord) . max ((((((({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_track_size }
                                    ) . apply_pin (_self) . get () . get () as f64) * (r#tmp_i_vertical_bar_20_page_size . clone () as f64)) as f64) / (((r#tmp_i_vertical_bar_20_maximum . clone () as f64) + (r#tmp_i_vertical_bar_20_page_size . clone () as f64)) as f64)) as sp :: Coord)) as f64) * (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                                ) as _ }
                             as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                        ) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_23_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ! false {
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_23_width }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) }
                     else {
                         (((2f64 as f64) + (((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_track_size }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_23_width }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) * (((- ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as f64) / (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_maximum }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_23_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if false {
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_23_height }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) }
                     else {
                         (((2f64 as f64) + (((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_track_size }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_23_height }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) * (((- ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as f64) / (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_maximum }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_29_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if true {
                         ((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (((2f64 as f64) * (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_pad }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) }
                     else {
                         ({
                             let r#tmp_i_horizontal_bar_26_maximum = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_maximum }
                            ) . apply_pin (_self) . get () . get () ;
                             ;
                             ((if ((r#tmp_i_horizontal_bar_26_maximum . clone () as f64) <= (((0f64 as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) as f64)) {
                                 0f64 }
                             else {
                                 ({
                                     let r#tmp_i_horizontal_bar_26_page_size = ({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_visible_width }
                                    ) . apply_pin (_self) . get () . get () ;
                                     ;
                                     (((32f64 as sp :: Coord) . max ((((({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_track_size }
                                    ) . apply_pin (_self) . get () . get () as f64) * (((r#tmp_i_horizontal_bar_26_page_size . clone () as f64) / (((r#tmp_i_horizontal_bar_26_maximum . clone () as f64) + (r#tmp_i_horizontal_bar_26_page_size . clone () as f64)) as f64)) as f64)) as sp :: Coord)) as f64) * (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                                ) as _ }
                             as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                        ) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_29_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ! true {
                         ((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (((2f64 as f64) * (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_pad }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) }
                     else {
                         ({
                             let r#tmp_i_horizontal_bar_26_maximum = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_maximum }
                            ) . apply_pin (_self) . get () . get () ;
                             ;
                             ((if ((r#tmp_i_horizontal_bar_26_maximum . clone () as f64) <= (((0f64 as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) as f64)) {
                                 0f64 }
                             else {
                                 ({
                                     let r#tmp_i_horizontal_bar_26_page_size = ({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_visible_width }
                                    ) . apply_pin (_self) . get () . get () ;
                                     ;
                                     (((32f64 as sp :: Coord) . max ((((((({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_track_size }
                                    ) . apply_pin (_self) . get () . get () as f64) * (r#tmp_i_horizontal_bar_26_page_size . clone () as f64)) as f64) / (((r#tmp_i_horizontal_bar_26_maximum . clone () as f64) + (r#tmp_i_horizontal_bar_26_page_size . clone () as f64)) as f64)) as sp :: Coord)) as f64) * (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                                ) as _ }
                             as f64) / (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                        ) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_29_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ! true {
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_29_width }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) }
                     else {
                         (((2f64 as f64) + (((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_track_size }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_29_width }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) * (((- ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as f64) / (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_maximum }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_29_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if true {
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_29_height }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) }
                     else {
                         (((2f64 as f64) + (((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_track_size }
                        ) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_29_height }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64) * (((- ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as f64) / (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_maximum }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_touch_area_24_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_touch_area_24_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_height }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_touch_area_30_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_touch_area_30_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_height }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_visible }
                    ) . apply_pin (_self) . get () {
                         ((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                        ) . apply_pin (_self) . get () [7usize] as f64) - (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_height }
                        ) . apply_pin (_self) . get () . get () as f64)) }
                     else {
                         (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                        ) . apply_pin (_self) . get () [7usize]) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_maximum }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_visible_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_pad }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_24 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (0f64) as _ }
                     as f64) == (1f64 as f64)) {
                         4f64 }
                     else {
                         (2f64) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_track_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if false {
                         ((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64)) }
                     else {
                         (((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_visible }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as f64) > (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_visible_height }
                    ) . apply_pin (_self) . get () . get () as f64))) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_24 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         20f64 }
                     else {
                         (12f64) as _ }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_12_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as f64) - (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) + (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: Item :: layout_info (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) + (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_preview_key }
            ) . apply_pin (_self) . set ({
                 (sp :: ModelRc :: new (sp :: VecModel :: < i32 > :: from (sp :: vec ! []))) as sp :: ModelRc < i32 > }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_preview_output }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_response_output }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_status_code_output }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("api2db")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (500f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294901760f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (10f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_3_padding = 12f64 ;
                         ;
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                        ) . apply_pin (_self) . get () [1usize] as f64) - (r#tmp_empty_3_padding . clone () as f64)) as f64) - (r#tmp_empty_3_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((((sp :: SharedString :: from ("")) + (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_error_output }
                    ) . apply_pin (_self) . get () . as_str ()))) + (sp :: SharedString :: from ("") . as_str ()))) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (75f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                    ) . color ())) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_5_padding = 12f64 ;
                         ;
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                        ) . apply_pin (_self) . get () [3usize] as f64) - (r#tmp_empty_5_padding . clone () as f64)) as f64) - (r#tmp_empty_5_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("API:")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
             + {
                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
             + {
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
             + {
                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
             + {
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_placeholder_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("api string")) as sp :: SharedString }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (300f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
                 + {
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_8 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294967295f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_8 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_8 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_5_padding = 12f64 ;
                         ;
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                        ) . apply_pin (_self) . get () [3usize] as f64) - (r#tmp_empty_5_padding . clone () as f64)) as f64) - (r#tmp_empty_5_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_8 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((((sp :: SharedString :: from ("")) + (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_status_code_output }
                    ) . apply_pin (_self) . get () . as_str ()))) + (sp :: SharedString :: from ("") . as_str ()))) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_8 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (75f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
                 + {
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_api_response }
                            ) . apply_pin (_self) . call (& (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
                             + {
                                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
                             + {
                                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (30f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Test Connection")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
                 + {
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_9_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
                 + {
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_9_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
                 + {
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_api_response_dl }
                            ) . apply_pin (_self) . call (& (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_response_output }
                            ) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (30f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Download")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
                 + {
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_9_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
                 + {
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_9_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
             + sp :: r#Flickable :: FIELD_OFFSETS . r#interactive) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_visible_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_visible_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_vertical_bar_visibility_19 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (! ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_visible }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_vertical_bar_20 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_24 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (0f64) as _ }
                     as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                        ) . with_alpha (0.2f64 as f32) }
                     else {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as _ }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_border_21 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_24 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (0f64) as _ }
                     as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                        ) . with_alpha (0.2f64 as f32) }
                     else {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_opacity_22 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (0.6f64) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_23 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_23 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4278190080f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_23 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((if false {
                         ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_23_height }
                        ) . apply_pin (_self) . get () . get () }
                     else {
                         (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_23_width }
                        ) . apply_pin (_self) . get () . get ()) as _ }
                     as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_23 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0.8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_24 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_24 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#moved) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ((true) && (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_24 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ())) {
                                 {
                                     ({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new (- (0f64 as sp :: Coord) . max (((({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_maximum }
                                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . min ((((({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_touch_area_24_pressed_value }
                                    ) . apply_pin (_self) . get () . get () as f64) + (if false {
                                         ((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_24 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_24 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_x) . apply_pin (_self) . get () . get () as f64)) as f64) * (((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_maximum }
                                        ) . apply_pin (_self) . get () . get () as f64) / (((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_track_size }
                                        ) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_23_width }
                                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64)) }
                                     else {
                                         (((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_24 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_24 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_y) . apply_pin (_self) . get () . get () as f64)) as f64) * (((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_maximum }
                                        ) . apply_pin (_self) . get () . get () as f64) / (((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_track_size }
                                        ) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_23_height }
                                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                                     as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_24 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#pointer_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#button) == (sp :: r#PointerEventButton :: r#Left))) && ((((args . 0 . clone ()) . r#kind) == (sp :: r#PointerEventKind :: r#Down)))) {
                                 {
                                     ({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_touch_area_24_pressed_value }
                                    ) . apply_pin (_self) . set (sp :: LogicalLength :: new (- ({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as sp :: Coord) as _) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_24 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#scroll_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let r#returned_expression1 = {
                                 let r#return_check_merge1 = if ((false) && ((((args . 0 . clone ()) . r#delta_x as f64) != (0f64 as f64)))) {
                                     (false , {
                                         ({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- ({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_maximum }
                                        ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (((0f64 as sp :: Coord) . min ((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as f64) + ((args . 0 . clone ()) . r#delta_x as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) ;
                                         sp :: r#EventResult :: r#Accept }
                                     ,) }
                                 else {
                                     (if ! ((! false) && ((((args . 0 . clone ()) . r#delta_y as f64) != (0f64 as f64)))) {
                                         {
                                             {
                                                 }
                                             ;
                                             (true , sp :: r#EventResult :: r#Reject ,) }
                                         }
                                     else {
                                         ((false , {
                                             ({
                                                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- ({
                                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_maximum }
                                            ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (((0f64 as sp :: Coord) . min ((((({
                                                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as f64) + ((args . 0 . clone ()) . r#delta_y as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) ;
                                             sp :: r#EventResult :: r#Accept }
                                         ,)) as _ }
                                    ) as _ }
                                 ;
                                 ;
                                 if (r#return_check_merge1 . clone ()) . 0 {
                                     ({
                                         sp :: r#EventResult :: r#Reject }
                                     , true , sp :: r#EventResult :: r#Reject ,) }
                                 else {
                                     ((sp :: r#EventResult :: r#Reject , false , (r#return_check_merge1 . clone ()) . 1 ,)) as _ }
                                 }
                             ;
                             ;
                             if (r#returned_expression1 . clone ()) . 1 {
                                 (r#returned_expression1 . clone ()) . 0 }
                             else {
                                 ((r#returned_expression1 . clone ()) . 2) as _ }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_25 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (! ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_visible }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_horizontal_bar_26 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_30 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (0f64) as _ }
                     as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                        ) . with_alpha (0.2f64 as f32) }
                     else {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as _ }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_border_27 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((if ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_30 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (0f64) as _ }
                     as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                        ) . with_alpha (0.2f64 as f32) }
                     else {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_opacity_28 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (0.6f64) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_29 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_29 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4278190080f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_29 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((if true {
                         ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_29_height }
                        ) . apply_pin (_self) . get () . get () }
                     else {
                         (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_29_width }
                        ) . apply_pin (_self) . get () . get ()) as _ }
                     as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_29 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0.8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_30 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_30 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#moved) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ((true) && (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_30 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ())) {
                                 {
                                     ({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new (- (0f64 as sp :: Coord) . max (((({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_maximum }
                                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . min ((((({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_touch_area_30_pressed_value }
                                    ) . apply_pin (_self) . get () . get () as f64) + (if true {
                                         ((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_30 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_30 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_x) . apply_pin (_self) . get () . get () as f64)) as f64) * (((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_maximum }
                                        ) . apply_pin (_self) . get () . get () as f64) / (((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_track_size }
                                        ) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_29_width }
                                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64)) }
                                     else {
                                         (((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_30 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_30 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_y) . apply_pin (_self) . get () . get () as f64)) as f64) * (((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_maximum }
                                        ) . apply_pin (_self) . get () . get () as f64) / (((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_track_size }
                                        ) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_29_height }
                                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                                     as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_30 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#pointer_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#button) == (sp :: r#PointerEventButton :: r#Left))) && ((((args . 0 . clone ()) . r#kind) == (sp :: r#PointerEventKind :: r#Down)))) {
                                 {
                                     ({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_touch_area_30_pressed_value }
                                    ) . apply_pin (_self) . set (sp :: LogicalLength :: new (- ({
                                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as sp :: Coord) as _) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_30 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#scroll_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let r#returned_expression2 = {
                                 let r#return_check_merge2 = if ((true) && ((((args . 0 . clone ()) . r#delta_x as f64) != (0f64 as f64)))) {
                                     (false , {
                                         ({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- ({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_maximum }
                                        ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (((0f64 as sp :: Coord) . min ((((({
                                             * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as f64) + ((args . 0 . clone ()) . r#delta_x as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) ;
                                         sp :: r#EventResult :: r#Accept }
                                     ,) }
                                 else {
                                     (if ! ((! true) && ((((args . 0 . clone ()) . r#delta_y as f64) != (0f64 as f64)))) {
                                         {
                                             {
                                                 }
                                             ;
                                             (true , sp :: r#EventResult :: r#Reject ,) }
                                         }
                                     else {
                                         ((false , {
                                             ({
                                                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- ({
                                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_maximum }
                                            ) . apply_pin (_self) . get () . get () as sp :: Coord) . max (((0f64 as sp :: Coord) . min ((((({
                                                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as f64) + ((args . 0 . clone ()) . r#delta_y as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) ;
                                             sp :: r#EventResult :: r#Accept }
                                         ,)) as _ }
                                    ) as _ }
                                 ;
                                 ;
                                 if (r#return_check_merge2 . clone ()) . 0 {
                                     ({
                                         sp :: r#EventResult :: r#Reject }
                                     , true , sp :: r#EventResult :: r#Reject ,) }
                                 else {
                                     ((sp :: r#EventResult :: r#Reject , false , (r#return_check_merge2 . clone ()) . 1 ,)) as _ }
                                 }
                             ;
                             ;
                             if (r#returned_expression2 . clone ()) . 1 {
                                 (r#returned_expression2 . clone ()) . 0 }
                             else {
                                 ((r#returned_expression2 . clone ()) . 2) as _ }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (30f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("DynamoDB")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
                 + {
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_31_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
                 + {
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_31_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (30f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("MongoDB")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
                 + {
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_31_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
                 + {
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_31_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (30f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Postgres")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
                 + {
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_31_layout_cache }
                    ) . apply_pin (_self) . get () [5usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
                 + {
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_31_layout_cache }
                    ) . apply_pin (_self) . get () [4usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_36 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                    ) . color ())) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_36 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_36 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_35_padding = 12f64 ;
                         ;
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                        ) . apply_pin (_self) . get () [11usize] as f64) - (r#tmp_empty_35_padding . clone () as f64)) as f64) - (r#tmp_empty_35_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_36 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((((sp :: SharedString :: from ("DB: ")) + (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_db_input }
                    ) . apply_pin (_self) . get () . as_str ()))) + (sp :: SharedString :: from ("") . as_str ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_36 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_35_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#db_string_37 }
             + {
                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
             + {
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#db_string_37 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#db_string_37 }
             + {
                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
             + {
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43_placeholder_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("db string")) as sp :: SharedString }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#db_string_37 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (300f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#db_string_37 }
                 + {
                     * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_35_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#db_string_37 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                    ) . color ())) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_38_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Schema:")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_38_padding = 12f64 ;
                         ;
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                        ) . apply_pin (_self) . get () [13usize] as f64) - (r#tmp_empty_38_padding . clone () as f64)) as f64) - (r#tmp_empty_38_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_41 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                    ) . color ())) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_41 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_41 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_40_padding = 12f64 ;
                         ;
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                        ) . apply_pin (_self) . get () [15usize] as f64) - (r#tmp_empty_40_padding . clone () as f64)) as f64) - (r#tmp_empty_40_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_41 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("TEST")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_41 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_40_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
                 + {
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_api_response }
                            ) . apply_pin (_self) . call (& (({
                                 InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
                             + {
                                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
                             + {
                                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (30f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Ingest")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
                 + {
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                    ) . apply_pin (_self) . get () [17usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
                 + {
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                    ) . apply_pin (_self) . get () [16usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
                 + {
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                    ) . apply_pin (_self) . get () [16usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#always_on_top) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_visibility_25_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_visibility_25_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_visibility_25_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_visibility_25_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_opacity_22_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_opacity_28_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_visibility_19_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_visibility_19_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_visibility_19_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_visibility_19_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#no_frame) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47__opacity_48_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_background_50_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_background_50_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_8 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_8 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_8 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_8 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_8 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_8 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_8 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_8 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_8 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_8 }
             + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_8 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53__opacity_54_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_checkable }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_focus_scope_79_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_touch_area_78_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_touch_area_78_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53__opacity_54_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_checkable }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_focus_scope_79_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_touch_area_78_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_touch_area_78_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
             + sp :: r#Flickable :: FIELD_OFFSETS . r#interactive) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_vertical_bar_visibility_19 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_vertical_bar_visibility_19 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_vertical_bar_visibility_19 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_vertical_bar_visibility_19 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_vertical_bar_visibility_19 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_23 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_24 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_24 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_25 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_25 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_25 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_25 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_25 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_29 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_30 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_30 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53__opacity_54_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_checkable }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_focus_scope_79_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_touch_area_78_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_touch_area_78_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53__opacity_54_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_checkable }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_focus_scope_79_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_touch_area_78_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_touch_area_78_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53__opacity_54_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_checkable }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_focus_scope_79_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_touch_area_78_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_touch_area_78_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_36 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_36 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_36 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_36 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_36 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_36 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_36 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_36 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_36 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#db_string_37 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47__opacity_48_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#db_string_37 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#db_string_37 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_background_50_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#db_string_37 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_i_background_50_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#db_string_37 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#db_string_37 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_41 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_41 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_41 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_41 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_41 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_41 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_41 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_41 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_41 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_41 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53__opacity_54_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_checkable }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_focus_scope_79_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_touch_area_78_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_i_touch_area_78_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text }
            ) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             InnerLineEdit_root_47 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#api_string_7 }
             . apply_pin (x)) ,) ;
             InnerButton_root_53 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_10 }
             . apply_pin (x)) ,) ;
             InnerButton_root_53 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_11 }
             . apply_pin (x)) ,) ;
             InnerButton_root_53 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_32 }
             . apply_pin (x)) ,) ;
             InnerButton_root_53 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_33 }
             . apply_pin (x)) ,) ;
             InnerButton_root_53 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_34 }
             . apply_pin (x)) ,) ;
             InnerLineEdit_root_47 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#db_string_37 }
             . apply_pin (x)) ,) ;
             InnerButton_root_53 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_42 }
             . apply_pin (x)) ,) ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 {
                     {
                         }
                     ;
                     {
                         }
                     }
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated_listview (|| {
                         InnerComponent_rectangle_16 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                     , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_visible_width }
                    ) . apply_pin (_self) . get () , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_visible_height }
                    ) . apply_pin (_self)) ;
                     _self . repeater0 . visit (order , visitor) }
                 1u32 ..= 4u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_10 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 1u32 , order , visitor) }
                 5u32 ..= 8u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_11 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 5u32 , order , visitor) }
                 9u32 ..= 12u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_32 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 9u32 , order , visitor) }
                 13u32 ..= 16u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_33 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 13u32 , order , visitor) }
                 17u32 ..= 20u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_34 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 17u32 , order , visitor) }
                 21u32 ..= 24u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_42 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 21u32 , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 500f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 500f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 750f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 750f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated_listview (|| {
                         InnerComponent_rectangle_16 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                     , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_visible_width }
                    ) . apply_pin (_self) . get () , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_visible_height }
                    ) . apply_pin (_self)) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 ..= 4u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_10 }
                     . apply_pin (_self) . subtree_range (dyn_index - 1u32) }
                 5u32 ..= 8u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_11 }
                     . apply_pin (_self) . subtree_range (dyn_index - 5u32) }
                 9u32 ..= 12u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_32 }
                     . apply_pin (_self) . subtree_range (dyn_index - 9u32) }
                 13u32 ..= 16u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_33 }
                     . apply_pin (_self) . subtree_range (dyn_index - 13u32) }
                 17u32 ..= 20u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_34 }
                     . apply_pin (_self) . subtree_range (dyn_index - 17u32) }
                 21u32 ..= 24u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_42 }
                     . apply_pin (_self) . subtree_range (dyn_index - 21u32) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated_listview (|| {
                         InnerComponent_rectangle_16 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                     , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_visible_width }
                    ) . apply_pin (_self) . get () , ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_visible_height }
                    ) . apply_pin (_self)) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 ..= 4u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_10 }
                     . apply_pin (_self) . subtree_component (dyn_index - 1u32 , subtree_index , result) }
                 5u32 ..= 8u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_11 }
                     . apply_pin (_self) . subtree_component (dyn_index - 5u32 , subtree_index , result) }
                 9u32 ..= 12u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_32 }
                     . apply_pin (_self) . subtree_component (dyn_index - 9u32 , subtree_index , result) }
                 13u32 ..= 16u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_33 }
                     . apply_pin (_self) . subtree_component (dyn_index - 13u32 , subtree_index , result) }
                 17u32 ..= 20u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_34 }
                     . apply_pin (_self) . subtree_component (dyn_index - 17u32 , subtree_index , result) }
                 21u32 ..= 24u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_42 }
                     . apply_pin (_self) . subtree_component (dyn_index - 21u32 , subtree_index , result) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (750f64 as sp :: Coord , 500f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 2u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 3u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [5usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [5usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord ,) , 4u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [7usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [7usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [6usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [6usize] as sp :: Coord ,) , 5u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [9usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [9usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [8usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [8usize] as sp :: Coord ,) , 6u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [11usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [11usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [10usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [10usize] as sp :: Coord ,) , 7u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [13usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [13usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [12usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [12usize] as sp :: Coord ,) , 8u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [15usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [15usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [14usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [14usize] as sp :: Coord ,) , 9u32 => (30f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [17usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                ) . apply_pin (_self) . get () [16usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [16usize] as sp :: Coord ,) , 10u32 => ({
                     let r#tmp_empty_3_padding = 12f64 ;
                     ;
                     ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                    ) . apply_pin (_self) . get () [1usize] as f64) - (r#tmp_empty_3_padding . clone () as f64)) as f64) - (r#tmp_empty_3_padding . clone () as f64)) }
                 as sp :: Coord , 75f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 11u32 => ({
                     let r#tmp_empty_5_padding = 12f64 ;
                     ;
                     ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                    ) . apply_pin (_self) . get () [3usize] as f64) - (r#tmp_empty_5_padding . clone () as f64)) as f64) - (r#tmp_empty_5_padding . clone () as f64)) }
                 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 12u32 => (20f64 as sp :: Coord , 300f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 13u32 => ({
                     let r#tmp_empty_5_padding = 12f64 ;
                     ;
                     ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                    ) . apply_pin (_self) . get () [3usize] as f64) - (r#tmp_empty_5_padding . clone () as f64)) as f64) - (r#tmp_empty_5_padding . clone () as f64)) }
                 as sp :: Coord , 75f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_5_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 21u32 => (30f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_9_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_9_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 22u32 => (30f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_9_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_9_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 39u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                ) . apply_pin (_self) . get () [7usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_12_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_12_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 40u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_visible_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_visible_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 2f64 as sp :: Coord , 2f64 as sp :: Coord ,) , 41u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_visibility_19_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_visibility_19_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_visibility_19_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_visibility_19_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 42u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_visibility_25_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_visibility_25_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_visibility_25_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_visibility_25_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 43u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 45u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord ,) , 46u32 => (if ! false {
                     ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_height }
                    ) . apply_pin (_self) . get () . get () }
                 else {
                     (0.8f64) as _ }
                 as sp :: Coord , if ! false {
                     0.8f64 }
                 else {
                     (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_width }
                    ) . apply_pin (_self) . get () . get ()) as _ }
                 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 47u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_23_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_23_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_23_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_23_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 48u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_vertical_bar_20_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_touch_area_24_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_touch_area_24_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 49u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_23_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_23_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_opacity_22_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_opacity_22_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 50u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 51u32 => (if ! true {
                     ({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_height }
                    ) . apply_pin (_self) . get () . get () }
                 else {
                     (0.8f64) as _ }
                 as sp :: Coord , if ! true {
                     0.8f64 }
                 else {
                     (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_width }
                    ) . apply_pin (_self) . get () . get ()) as _ }
                 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 52u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_29_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_29_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_29_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_29_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 53u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_horizontal_bar_26_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_touch_area_30_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_touch_area_30_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 54u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_29_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_29_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_opacity_28_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_i_thumb_opacity_28_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 55u32 => (30f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_31_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_31_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 56u32 => (30f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_31_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_31_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 57u32 => (30f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_31_layout_cache }
                ) . apply_pin (_self) . get () [5usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_31_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 82u32 => ({
                     let r#tmp_empty_35_padding = 12f64 ;
                     ;
                     ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                    ) . apply_pin (_self) . get () [11usize] as f64) - (r#tmp_empty_35_padding . clone () as f64)) as f64) - (r#tmp_empty_35_padding . clone () as f64)) }
                 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_35_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_35_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 83u32 => (20f64 as sp :: Coord , 300f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_35_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 91u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_38_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , {
                     let r#tmp_empty_38_padding = 12f64 ;
                     ;
                     ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_h }
                    ) . apply_pin (_self) . get () [13usize] as f64) - (r#tmp_empty_38_padding . clone () as f64)) as f64) - (r#tmp_empty_38_padding . clone () as f64)) }
                 as sp :: Coord , 12f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_38_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 92u32 => ({
                     let r#tmp_empty_40_padding = 12f64 ;
                     ;
                     ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache_v }
                    ) . apply_pin (_self) . get () [15usize] as f64) - (r#tmp_empty_40_padding . clone () as f64)) as f64) - (r#tmp_empty_40_padding . clone () as f64)) }
                 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_40_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_40_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 14u32 ..= 20u32 => return {
                     * & Self :: FIELD_OFFSETS . r#api_string_7 }
                 . apply_pin (_self) . item_geometry (index - 14u32 + 1) , 23u32 ..= 30u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_10 }
                 . apply_pin (_self) . item_geometry (index - 23u32 + 1) , 31u32 ..= 38u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_11 }
                 . apply_pin (_self) . item_geometry (index - 31u32 + 1) , 58u32 ..= 65u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_32 }
                 . apply_pin (_self) . item_geometry (index - 58u32 + 1) , 66u32 ..= 73u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_33 }
                 . apply_pin (_self) . item_geometry (index - 66u32 + 1) , 74u32 ..= 81u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_34 }
                 . apply_pin (_self) . item_geometry (index - 74u32 + 1) , 84u32 ..= 90u32 => return {
                     * & Self :: FIELD_OFFSETS . r#db_string_37 }
                 . apply_pin (_self) . item_geometry (index - 84u32 + 1) , 93u32 ..= 100u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_42 }
                 . apply_pin (_self) . item_geometry (index - 93u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 9u32 => sp :: r#AccessibleRole :: r#Button , 10u32 => sp :: r#AccessibleRole :: r#Text , 11u32 => sp :: r#AccessibleRole :: r#Text , 13u32 => sp :: r#AccessibleRole :: r#Text , 21u32 => sp :: r#AccessibleRole :: r#Button , 22u32 => sp :: r#AccessibleRole :: r#Button , 55u32 => sp :: r#AccessibleRole :: r#Button , 56u32 => sp :: r#AccessibleRole :: r#Button , 57u32 => sp :: r#AccessibleRole :: r#Button , 82u32 => sp :: r#AccessibleRole :: r#Text , 91u32 => sp :: r#AccessibleRole :: r#Text , 92u32 => sp :: r#AccessibleRole :: r#Text , 12u32 => {
                     * & Self :: FIELD_OFFSETS . r#api_string_7 }
                 . apply_pin (_self) . accessible_role (0) , 14u32 ..= 20u32 => {
                     * & Self :: FIELD_OFFSETS . r#api_string_7 }
                 . apply_pin (_self) . accessible_role (index - 14u32 + 1) , 21u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_10 }
                 . apply_pin (_self) . accessible_role (0) , 23u32 ..= 30u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_10 }
                 . apply_pin (_self) . accessible_role (index - 23u32 + 1) , 22u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_11 }
                 . apply_pin (_self) . accessible_role (0) , 31u32 ..= 38u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_11 }
                 . apply_pin (_self) . accessible_role (index - 31u32 + 1) , 55u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_32 }
                 . apply_pin (_self) . accessible_role (0) , 58u32 ..= 65u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_32 }
                 . apply_pin (_self) . accessible_role (index - 58u32 + 1) , 56u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_33 }
                 . apply_pin (_self) . accessible_role (0) , 66u32 ..= 73u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_33 }
                 . apply_pin (_self) . accessible_role (index - 66u32 + 1) , 57u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_34 }
                 . apply_pin (_self) . accessible_role (0) , 74u32 ..= 81u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_34 }
                 . apply_pin (_self) . accessible_role (index - 74u32 + 1) , 83u32 => {
                     * & Self :: FIELD_OFFSETS . r#db_string_37 }
                 . apply_pin (_self) . accessible_role (0) , 84u32 ..= 90u32 => {
                     * & Self :: FIELD_OFFSETS . r#db_string_37 }
                 . apply_pin (_self) . accessible_role (index - 84u32 + 1) , 9u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_42 }
                 . apply_pin (_self) . accessible_role (0) , 93u32 ..= 100u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_42 }
                 . apply_pin (_self) . accessible_role (index - 93u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (9u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
                 + {
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text }
                ) . apply_pin (_self) . get () , (10u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (11u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("API:") , (13u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_8 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (21u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
                 + {
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text }
                ) . apply_pin (_self) . get () , (22u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
                 + {
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text }
                ) . apply_pin (_self) . get () , (55u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
                 + {
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text }
                ) . apply_pin (_self) . get () , (56u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
                 + {
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text }
                ) . apply_pin (_self) . get () , (57u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
                 + {
                     * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53_text }
                ) . apply_pin (_self) . get () , (82u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_36 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (91u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("Schema:") , (92u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("TEST") , (12u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#api_string_7 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (14u32 ..= 20u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#api_string_7 }
                 . apply_pin (_self) . accessible_string_property (index - 14u32 + 1 , what) , (21u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_10 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (23u32 ..= 30u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_10 }
                 . apply_pin (_self) . accessible_string_property (index - 23u32 + 1 , what) , (22u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_11 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (31u32 ..= 38u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_11 }
                 . apply_pin (_self) . accessible_string_property (index - 31u32 + 1 , what) , (55u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_32 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (58u32 ..= 65u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_32 }
                 . apply_pin (_self) . accessible_string_property (index - 58u32 + 1 , what) , (56u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_33 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (66u32 ..= 73u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_33 }
                 . apply_pin (_self) . accessible_string_property (index - 66u32 + 1 , what) , (57u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_34 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (74u32 ..= 81u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_34 }
                 . apply_pin (_self) . accessible_string_property (index - 74u32 + 1 , what) , (83u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#db_string_37 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (84u32 ..= 90u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#db_string_37 }
                 . apply_pin (_self) . accessible_string_property (index - 84u32 + 1 , what) , (9u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_42 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (93u32 ..= 100u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_42 }
                 . apply_pin (_self) . accessible_string_property (index - 93u32 + 1 , what) , _ => :: core :: default :: Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_rectangle_16 {
         r#rectangle_16 : sp :: r#Empty , r#text_17 : sp :: r#Text , r#model_data : sp :: Property < i32 > , r#model_index : sp :: Property < i32 > , r#rectangle_16_height : sp :: Property < sp :: LogicalLength > , r#rectangle_16_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#rectangle_16_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#rectangle_16_text_17_min_height : sp :: Property < sp :: LogicalLength > , r#rectangle_16_text_17_min_width : sp :: Property < sp :: LogicalLength > , r#rectangle_16_text_17_preferred_height : sp :: Property < sp :: LogicalLength > , r#rectangle_16_text_17_preferred_width : sp :: Property < sp :: LogicalLength > , r#rectangle_16_text_17_x : sp :: Property < sp :: LogicalLength > , r#rectangle_16_width : sp :: Property < sp :: LogicalLength > , r#rectangle_16_x : sp :: Property < sp :: LogicalLength > , r#rectangle_16_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_rectangle_16 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_rectangle_16 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             ({
                 * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (15f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#text_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#text_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_text_17_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#text_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_text_17_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#text_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_text_17_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#text_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_text_17_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#text_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_text_17_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_visible_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (({
                         * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#text_17 }
                     + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_visible_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#text_17 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_80 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_80 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                    ) . color ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#text_17 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_text_17_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_text_17_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#text_17 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: SharedString :: from (sp :: format ! ("{}" , ({
                         * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#model_data }
                    ) . apply_pin (_self) . get ()) . as_str ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#text_17 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_text_17_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_text_17_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#text_17 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => ({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_layoutinfo_h }
                ) . apply_pin (_self) . get () , sp :: Orientation :: Vertical => ({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             ({
                 * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#model_index }
            ) . apply_pin (_self) . get () as usize }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (15f64 as sp :: Coord , (InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_13_visible_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#text_17 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#text_17 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_text_17_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ((((15f64 as f64) - (({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#text_17 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (1u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from (sp :: format ! ("{}" , ({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#model_data }
                ) . apply_pin (_self) . get ()) . as_str ()) , _ => :: core :: default :: Default :: default () , }
             }
         }
     impl InnerComponent_rectangle_16 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow >) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerAppWindow > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_item_tree (& self_dyn_rc , (* & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap ()) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             2usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_rectangle_16 , sp :: ItemVTable , sp :: AllowPin > ;
             2usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#text_17 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_rectangle_16) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_rectangle_16 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_rectangle_16 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_rectangle_16 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_rectangle_16 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 44u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_rectangle_16 {
         type Data = i32 ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             ({
                 * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#model_index }
            ) . apply_pin (_self) . set (_index as _) ;
             ({
                 * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#model_data }
            ) . apply_pin (_self) . set (_data) ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_rectangle_16 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn listview_layout (self : core :: pin :: Pin < & Self > , offset_y : & mut sp :: LogicalLength , viewport_width : core :: pin :: Pin < & sp :: Property < sp :: LogicalLength >> ,) {
             let _self = self ;
             let vp_w = viewport_width . get () ;
             ({
                 * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_y }
            ) . apply_pin (_self) . set (* offset_y) ;
             * offset_y += ({
                 * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_height }
            ) . apply_pin (_self) . get () ;
             let w = ({
                 * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_width }
            ) . apply_pin (_self) . get () ;
             if vp_w < w {
                 viewport_width . set (w) ;
                 }
             }
         }
     impl InnerAppWindow {
         pub fn new () -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_item_tree (& self_dyn_rc , (* & self_rc) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & self_rc , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             101usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 9u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 10u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 11u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 21u32 , parent_index : 0u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 39u32 , parent_index : 0u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 55u32 , parent_index : 0u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 82u32 , parent_index : 0u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 91u32 , parent_index : 0u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 92u32 , parent_index : 0u32 , item_array_index : 8u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 7u32 , children_index : 93u32 , parent_index : 0u32 , item_array_index : 9u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 11u32 , parent_index : 1u32 , item_array_index : 10u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 14u32 , parent_index : 2u32 , item_array_index : 11u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 14u32 , parent_index : 2u32 , item_array_index : 12u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 21u32 , parent_index : 2u32 , item_array_index : 13u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 17u32 , parent_index : 12u32 , item_array_index : 14u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 18u32 , parent_index : 12u32 , item_array_index : 15u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 18u32 , parent_index : 12u32 , item_array_index : 16u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 18u32 , parent_index : 14u32 , item_array_index : 17u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 19u32 , parent_index : 16u32 , item_array_index : 18u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 21u32 , parent_index : 18u32 , item_array_index : 19u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 21u32 , parent_index : 18u32 , item_array_index : 20u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 7u32 , children_index : 23u32 , parent_index : 3u32 , item_array_index : 21u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 7u32 , children_index : 31u32 , parent_index : 3u32 , item_array_index : 22u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 30u32 , parent_index : 21u32 , item_array_index : 23u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 1u32 , parent_index : 21u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 2u32 , parent_index : 21u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 3u32 , parent_index : 21u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 4u32 , parent_index : 21u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 31u32 , parent_index : 21u32 , item_array_index : 24u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 31u32 , parent_index : 21u32 , item_array_index : 25u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 31u32 , parent_index : 23u32 , item_array_index : 26u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 38u32 , parent_index : 22u32 , item_array_index : 27u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 5u32 , parent_index : 22u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 6u32 , parent_index : 22u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 7u32 , parent_index : 22u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 8u32 , parent_index : 22u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 39u32 , parent_index : 22u32 , item_array_index : 28u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 39u32 , parent_index : 22u32 , item_array_index : 29u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 39u32 , parent_index : 31u32 , item_array_index : 30u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 40u32 , parent_index : 4u32 , item_array_index : 31u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 43u32 , parent_index : 39u32 , item_array_index : 32u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 45u32 , parent_index : 39u32 , item_array_index : 33u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 50u32 , parent_index : 39u32 , item_array_index : 34u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 44u32 , parent_index : 40u32 , item_array_index : 35u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 43u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 46u32 , parent_index : 41u32 , item_array_index : 36u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 49u32 , parent_index : 45u32 , item_array_index : 37u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 49u32 , parent_index : 45u32 , item_array_index : 38u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 50u32 , parent_index : 45u32 , item_array_index : 39u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 50u32 , parent_index : 47u32 , item_array_index : 40u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 51u32 , parent_index : 42u32 , item_array_index : 41u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 54u32 , parent_index : 50u32 , item_array_index : 42u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 54u32 , parent_index : 50u32 , item_array_index : 43u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 55u32 , parent_index : 50u32 , item_array_index : 44u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 55u32 , parent_index : 52u32 , item_array_index : 45u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 7u32 , children_index : 58u32 , parent_index : 5u32 , item_array_index : 46u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 7u32 , children_index : 66u32 , parent_index : 5u32 , item_array_index : 47u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 7u32 , children_index : 74u32 , parent_index : 5u32 , item_array_index : 48u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 65u32 , parent_index : 55u32 , item_array_index : 49u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 9u32 , parent_index : 55u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 10u32 , parent_index : 55u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 11u32 , parent_index : 55u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 12u32 , parent_index : 55u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 66u32 , parent_index : 55u32 , item_array_index : 50u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 66u32 , parent_index : 55u32 , item_array_index : 51u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 66u32 , parent_index : 58u32 , item_array_index : 52u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 73u32 , parent_index : 56u32 , item_array_index : 53u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 13u32 , parent_index : 56u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 14u32 , parent_index : 56u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 15u32 , parent_index : 56u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 16u32 , parent_index : 56u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 74u32 , parent_index : 56u32 , item_array_index : 54u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 74u32 , parent_index : 56u32 , item_array_index : 55u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 74u32 , parent_index : 66u32 , item_array_index : 56u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 81u32 , parent_index : 57u32 , item_array_index : 57u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 17u32 , parent_index : 57u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 18u32 , parent_index : 57u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 19u32 , parent_index : 57u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 20u32 , parent_index : 57u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 82u32 , parent_index : 57u32 , item_array_index : 58u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 82u32 , parent_index : 57u32 , item_array_index : 59u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 82u32 , parent_index : 74u32 , item_array_index : 60u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 84u32 , parent_index : 6u32 , item_array_index : 61u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 84u32 , parent_index : 6u32 , item_array_index : 62u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 87u32 , parent_index : 83u32 , item_array_index : 63u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 88u32 , parent_index : 83u32 , item_array_index : 64u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 88u32 , parent_index : 83u32 , item_array_index : 65u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 88u32 , parent_index : 84u32 , item_array_index : 66u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 89u32 , parent_index : 86u32 , item_array_index : 67u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 91u32 , parent_index : 88u32 , item_array_index : 68u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 91u32 , parent_index : 88u32 , item_array_index : 69u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 92u32 , parent_index : 7u32 , item_array_index : 70u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 93u32 , parent_index : 8u32 , item_array_index : 71u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 100u32 , parent_index : 9u32 , item_array_index : 72u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 21u32 , parent_index : 9u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 22u32 , parent_index : 9u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 23u32 , parent_index : 9u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 24u32 , parent_index : 9u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 101u32 , parent_index : 9u32 , item_array_index : 73u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 101u32 , parent_index : 9u32 , item_array_index : 74u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 101u32 , parent_index : 93u32 , item_array_index : 75u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerAppWindow , sp :: ItemVTable , sp :: AllowPin > ;
             76usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_3 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_5 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_9 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_12 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_31 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_35 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_38 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_40 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_6 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_8 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#_opacity_48 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_background_50 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
             + {
                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
             + {
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#rectangle_49 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
             + {
                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
             + {
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_clip_44 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
             + {
                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
             + {
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#api_string_7 }
             + {
                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
             + {
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#_opacity_54 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_touch_area_78 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_focus_scope_79 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_10 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#rectangle_55 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#_opacity_54 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_touch_area_78 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_focus_scope_79 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_11 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#rectangle_55 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_13 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_14 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_vertical_bar_visibility_19 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_horizontal_bar_visibility_25 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_flickable_viewport_15 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_vertical_bar_20 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_border_21 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_opacity_22 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_24 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_23 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_horizontal_bar_26 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_border_27 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_opacity_28 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_touch_area_30 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#i_thumb_29 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#root_53 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#_opacity_54 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_touch_area_78 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_focus_scope_79 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_32 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#rectangle_55 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#_opacity_54 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_touch_area_78 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_focus_scope_79 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_33 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#rectangle_55 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#_opacity_54 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_touch_area_78 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_focus_scope_79 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_34 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#rectangle_55 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_36 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#db_string_37 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#root_47 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#db_string_37 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#_opacity_48 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#db_string_37 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_background_50 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#db_string_37 }
             + {
                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
             + {
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_43 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#db_string_37 }
             + {
                 * & InnerLineEdit_root_47 :: FIELD_OFFSETS . r#rectangle_49 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#db_string_37 }
             + {
                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
             + {
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#root_clip_44 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#db_string_37 }
             + {
                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
             + {
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_placeholder_45 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#db_string_37 }
             + {
                 InnerLineEdit_root_47 :: FIELD_OFFSETS . r#i_base_52 }
             + {
                 * & InnerLineEditBase_root_43 :: FIELD_OFFSETS . r#i_text_input_46 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_39 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_41 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#_opacity_54 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_touch_area_78 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#i_focus_scope_79 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_42 }
             + {
                 * & InnerButton_root_53 :: FIELD_OFFSETS . r#rectangle_55 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             sp :: Rc :: clone (self . window_adapter_ref () . unwrap ()) }
         fn window_adapter_ref (& self ,) -> sp :: Result < & sp :: Rc < dyn sp :: WindowAdapter > , slint :: PlatformError > {
             self . window_adapter_ . get_or_try_init (|| {
                 let adapter = slint :: private_unstable_api :: create_window_adapter () ? ;
                 let self_rc = sp :: VRcMapped :: origin (& self . self_weak . get () . unwrap () . upgrade () . unwrap () ,) ;
                 sp :: WindowInner :: from_pub (adapter . window ()) . set_component (& self_rc) ;
                 core :: result :: Result :: Ok (adapter) }
            ) }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . window_adapter_ . get () . cloned () }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerAppWindow) ;
         }
     ;
     impl sp :: PinnedDrop for InnerAppWindow {
         fn drop (self : core :: pin :: Pin < & mut InnerAppWindow >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerAppWindow {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerAppWindow > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             false }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     pub struct r#AppWindow (sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) ;
     impl r#AppWindow {
         pub fn new () -> core :: result :: Result < Self , slint :: PlatformError > {
             let inner = InnerAppWindow :: new () ? ;
             inner . globals . global_ColorSchemeSelector_80 . clone () . init (& inner) ;
             inner . globals . global_CupertinoPalette_81 . clone () . init (& inner) ;
             inner . globals . global_StyleMetrics_82 . clone () . init (& inner) ;
             InnerAppWindow :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             core :: result :: Result :: Ok (Self (inner)) }
         # [allow (dead_code)] pub fn get_api_input (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_api_input }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_api_input (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_api_input }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_api_response (& self , arg_0 : sp :: SharedString ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_api_response }
            ) . apply_pin (_self) . call (& (arg_0 ,)) }
         # [allow (dead_code)] pub fn on_api_response (& self , mut f : impl FnMut (sp :: SharedString) -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_api_response }
            ) . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone ())) }
         # [allow (dead_code)] pub fn invoke_api_response_dl (& self , arg_0 : sp :: SharedString ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_api_response_dl }
            ) . apply_pin (_self) . call (& (arg_0 ,)) }
         # [allow (dead_code)] pub fn on_api_response_dl (& self , mut f : impl FnMut (sp :: SharedString) -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_api_response_dl }
            ) . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone ())) }
         # [allow (dead_code)] pub fn get_db_input (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_db_input }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_db_input (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_db_input }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_error_output (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_error_output }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_error_output (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_error_output }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_preview_key (& self) -> sp :: ModelRc < i32 > {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_preview_key }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_preview_key (& self , value : sp :: ModelRc < i32 >) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_preview_key }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_preview_output (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_preview_output }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_preview_output (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_preview_output }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_response_output (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_response_output }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_response_output (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_response_output }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_status_code_output (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_status_code_output }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_status_code_output (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_status_code_output }
            ) . apply_pin (_self) . set (value as _) }
         }
     impl From < r#AppWindow > for sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > {
         fn from (value : r#AppWindow) -> Self {
             value . 0 }
         }
     impl slint :: ComponentHandle for r#AppWindow {
         type Inner = InnerAppWindow ;
         fn as_weak (& self) -> slint :: Weak < Self > {
             slint :: Weak :: new (& self . 0) }
         fn clone_strong (& self) -> Self {
             Self (self . 0 . clone ()) }
         fn from_inner (inner : sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) -> Self {
             Self (inner) }
         fn run (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . show () ? ;
             slint :: run_event_loop () ? ;
             self . hide () ? ;
             core :: result :: Result :: Ok (()) }
         fn show (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . window_adapter_ref () ? . window () . show () }
         fn hide (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . window_adapter_ref () ? . window () . hide () }
         fn window (& self) -> & slint :: Window {
             self . 0 . window_adapter_ref () . unwrap () . window () }
         fn global < 'a , T : slint :: Global < 'a , Self >> (& 'a self) -> T {
             T :: get (& self) }
         }
     # [allow (dead_code)] struct Globals_AppWindow {
         global_ColorSchemeSelector_80 : :: core :: pin :: Pin < sp :: Rc < InnerColorSchemeSelector_80 >> , global_CupertinoPalette_81 : :: core :: pin :: Pin < sp :: Rc < InnerCupertinoPalette_81 >> , global_StyleMetrics_82 : :: core :: pin :: Pin < sp :: Rc < InnerStyleMetrics_82 >> , }
     impl :: core :: default :: Default for Globals_AppWindow {
         fn default () -> Self {
             Self {
                 global_ColorSchemeSelector_80 : InnerColorSchemeSelector_80 :: new () , global_CupertinoPalette_81 : InnerCupertinoPalette_81 :: new () , global_StyleMetrics_82 : InnerStyleMetrics_82 :: new () , }
             }
         }
     const _THE_SAME_VERSION_MUST_BE_USED_FOR_THE_COMPILER_AND_THE_RUNTIME : slint :: VersionCheck_1_5_1 = slint :: VersionCheck_1_5_1 ;
     }
 # [allow (unused_imports)] pub use slint_generatedAppWindow :: {
     r#AppWindow , r#TextStyle }
 ;
 # [allow (unused_imports)] pub use slint :: {
     ComponentHandle as _ , Global as _ , ModelExt as _ }
 ;
