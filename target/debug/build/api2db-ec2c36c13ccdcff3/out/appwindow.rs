# [allow (non_snake_case)] # [allow (non_camel_case_types)] # [allow (clippy :: style)] # [allow (clippy :: complexity)] # [allow (unused_braces , unused_parens)] # [allow (clippy :: erasing_op)] # [allow (clippy :: approx_constant)] # [allow (clippy :: eq_op)] # [allow (clippy :: cmp_owned)] # [allow (clippy :: redundant_clone)] # [allow (clippy :: overly_complex_bool_expr)] mod slint_generatedAppWindow {
     use slint :: private_unstable_api :: re_exports as sp ;
     # [allow (unused_imports)] use sp :: {
         RepeatedItemTree as _ , ModelExt as _ , Model as _ , Float as _ }
     ;
     # [derive (Default , PartialEq , Debug , Clone)] pub struct r#TextStyle {
         pub r#font_size : f32 , pub r#font_weight : i32 }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerColorSchemeSelector_45 {
         r#dark_color_scheme : sp :: Property < bool > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , }
     impl InnerColorSchemeSelector_45 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) {
             # ! [allow (unused)] self . root . set (sp :: VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . dark_color_scheme ()) as _ }
                ) ;
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerCupertinoPalette_46 {
         root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , }
     impl InnerCupertinoPalette_46 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) {
             # ! [allow (unused)] self . root . set (sp :: VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerStyleMetrics_47 {
         root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , }
     impl InnerStyleMetrics_47 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) {
             # ! [allow (unused)] self . root . set (sp :: VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerLineEditBase_root_8 {
         r#root_8 : sp :: r#Empty , r#root_clip_9 : sp :: r#Clip , r#i_placeholder_10 : sp :: r#Text , r#i_text_input_11 : sp :: r#TextInput , r#root_8_has_focus : sp :: Property < bool > , r#root_8_height : sp :: Property < sp :: LogicalLength > , r#root_8_i_placeholder_10_horizontal_stretch : sp :: Property < f32 > , r#root_8_i_placeholder_10_max_height : sp :: Property < sp :: LogicalLength > , r#root_8_i_placeholder_10_max_width : sp :: Property < sp :: LogicalLength > , r#root_8_i_placeholder_10_min_height : sp :: Property < sp :: LogicalLength > , r#root_8_i_placeholder_10_min_width : sp :: Property < sp :: LogicalLength > , r#root_8_i_placeholder_10_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_8_i_placeholder_10_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_8_i_placeholder_10_vertical_stretch : sp :: Property < f32 > , r#root_8_i_placeholder_10_x : sp :: Property < sp :: LogicalLength > , r#root_8_i_placeholder_10_y : sp :: Property < sp :: LogicalLength > , r#root_8_i_text_input_11_computed_x : sp :: Property < sp :: LogicalLength > , r#root_8_i_text_input_11_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_8_i_text_input_11_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_8_i_text_input_11_x : sp :: Property < sp :: LogicalLength > , r#root_8_i_text_input_11_y : sp :: Property < sp :: LogicalLength > , r#root_8_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_8_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_8_margin : sp :: Property < sp :: LogicalLength > , r#root_8_placeholder_color : sp :: Property < slint :: Brush > , r#root_8_placeholder_text : sp :: Property < sp :: SharedString > , r#root_8_root_clip_9_x : sp :: Property < sp :: LogicalLength > , r#root_8_root_clip_9_y : sp :: Property < sp :: LogicalLength > , r#root_8_text_color : sp :: Property < slint :: Brush > , r#root_8_width : sp :: Property < sp :: LogicalLength > , r#root_8_x : sp :: Property < sp :: LogicalLength > , r#root_8_y : sp :: Property < sp :: LogicalLength > , r#root_8_accepted : sp :: Callback < (sp :: SharedString ,) , () > , r#root_8_edited : sp :: Callback < (sp :: SharedString ,) , () > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerLineEditBase_root_8 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerLineEditBase_root_8 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_has_focus }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_max_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_vertical_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_text_input_11_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_text_input_11_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_text_input_11_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min (((((((({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) - (1f64 as f64)) as sp :: Coord) . max ((({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_text_input_11_computed_x }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_layoutinfo_h }
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
                             * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_max_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_min_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_layoutinfo_v }
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
                             * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_max_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_min_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_preferred_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_vertical_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_clip_9 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_placeholder_color }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((((({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ()) == (sp :: SharedString :: from ("")))) && (((({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#preedit_text) . apply_pin (_self) . get ()) == (sp :: SharedString :: from (""))))) {
                         ({
                             * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_placeholder_text }
                        ) . apply_pin (_self) . get () }
                     else {
                         (sp :: SharedString :: from ("")) as _ }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#accepted) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_accepted }
                            ) . apply_pin (_self) . call (& (({
                                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_text_color }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#cursor_position_changed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#x as f64) + (({
                                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_text_input_11_computed_x }
                            ) . apply_pin (_self) . get () . get () as f64)) as f64) < (({
                                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_margin }
                            ) . apply_pin (_self) . get () . get () as f64)) {
                                 {
                                     ({
                                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_text_input_11_computed_x }
                                    ) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- (args . 0 . clone ()) . r#x as f64) + (({
                                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_margin }
                                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord) as _) }
                                 }
                             else {
                                 (if (((((args . 0 . clone ()) . r#x as f64) + (({
                                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_text_input_11_computed_x }
                                ) . apply_pin (_self) . get () . get () as f64)) as f64) > (((((({
                                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_width }
                                ) . apply_pin (_self) . get () . get () as f64) - (({
                                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_margin }
                                ) . apply_pin (_self) . get () . get () as f64)) as f64) - (1f64 as f64)) as f64)) {
                                     {
                                         ({
                                             * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_text_input_11_computed_x }
                                        ) . apply_pin (_self) . set (sp :: LogicalLength :: new (((((((({
                                             * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_width }
                                        ) . apply_pin (_self) . get () . get () as f64) - ((args . 0 . clone ()) . r#x as f64)) as f64) - (({
                                             * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_margin }
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
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#edited) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_edited }
                            ) . apply_pin (_self) . call (& (({
                                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (false) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Color :: from_argb_encoded (4286611584f64 as u32)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (1f64 as f64)) as sp :: Coord) . max ((({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_text_input_11_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_text_input_11_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_root_clip_9_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_root_clip_9_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_clip_9 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_clip_9 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_clip_9 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_clip_9 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_clip_9 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
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
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (50f64 as sp :: Coord) . max ((({
                             * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_min_width }
                        ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = ({
                             * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_text_input_11_preferred_height }
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
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_root_clip_9_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_root_clip_9_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 2u32 => (((1f64 as f64) * (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_height }
                ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord , ((1f64 as f64) * (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_width }
                ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord , ({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 3u32 => (((1f64 as f64) * (({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_height }
                ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord , ({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_text_input_11_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_text_input_11_y }
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
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , _ => :: core :: default :: Default :: default () , }
             }
         # [allow (dead_code , unused)] pub fn r#fn_clear_selection (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ({
                 ({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                ) . apply_pin (_self) . r#clear_selection ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) }
            ) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_copy (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ({
                 ({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                ) . apply_pin (_self) . r#copy ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) }
            ) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_cut (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ({
                 ({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
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
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                ) . apply_pin (_self) . r#paste ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) }
            ) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_select_all (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ({
                 ({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                ) . apply_pin (_self) . r#select_all ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1)) }
            ) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_set_selection_offsets (self : :: core :: pin :: Pin < & Self > , arg_0 : i32 , arg_1 : i32 ,) -> () {
             let _self = self ;
             let args = (arg_0 , arg_1 ,) ;
             ({
                 ({
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                ) . apply_pin (_self) . set_selection_offsets ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1) , args . 0 . clone () as i32 , args . 1 . clone () as i32) }
            ) ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerLineEdit_root_12 {
         r#root_12 : sp :: r#Empty , r#_opacity_13 : sp :: r#Opacity , r#rectangle_14 : sp :: r#Rectangle , r#i_background_15 : sp :: r#BasicBorderRectangle , r#i_base_17 : InnerLineEditBase_root_8 , r#root_12__opacity_13_dummy : sp :: Property < sp :: LogicalLength > , r#root_12_height : sp :: Property < sp :: LogicalLength > , r#root_12_i_background_15_x : sp :: Property < sp :: LogicalLength > , r#root_12_i_background_15_y : sp :: Property < sp :: LogicalLength > , r#root_12_i_layout_16_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_12_i_layout_16_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_12_i_layout_16_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_12_rectangle_14_x : sp :: Property < sp :: LogicalLength > , r#root_12_rectangle_14_y : sp :: Property < sp :: LogicalLength > , r#root_12_state : sp :: Property < i32 > , r#root_12_width : sp :: Property < sp :: LogicalLength > , r#root_12_x : sp :: Property < sp :: LogicalLength > , r#root_12_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerLineEdit_root_12 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerLineEdit_root_12 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerLineEditBase_root_8 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#i_base_17 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 3u32 - 1 , tree_index_of_first_child + 5u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_i_layout_16_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
                                 + {
                                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (50f64 as sp :: Coord) . max ((({
                                         InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
                                     + {
                                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_min_width }
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
                             * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_width }
                        ) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_i_layout_16_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
                             + {
                                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (50f64 as sp :: Coord) . max ((({
                                     InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
                                 + {
                                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_min_width }
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
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_i_layout_16_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
                             + {
                                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ({
                                     InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
                                 + {
                                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_text_input_11_preferred_height }
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
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_rectangle_14_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (((({
                         * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_width }
                    ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_rectangle_14_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_height }
                    ) . apply_pin (_self) . get () . get () as f64) - (((({
                         * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_height }
                    ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ! ({
                         InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
                     + {
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (if ({
                             InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
                         + {
                             * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_has_focus }
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
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#_opacity_13 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((if ({
                         InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
                     + {
                         * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_has_focus }
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
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#rectangle_14 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_45 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4282940159f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4284916195f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_background_15 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_12_state = ({
                             * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_root_12_state . clone () as f64) == (1f64 as f64)) {
                             slint :: Brush :: SolidColor (if InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_45 . as_ref ()) . get () {
                                 sp :: Color :: from_argb_encoded (3009503585f64 as u32) }
                             else {
                                 (sp :: Color :: from_argb_encoded (3019898879f64 as u32)) as _ }
                            ) }
                         else {
                             (if ((r#tmp_root_12_state . clone () as f64) == (2f64 as f64)) {
                                 slint :: Brush :: SolidColor (if InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_45 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (4284572001f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                                ) }
                             else {
                                 (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_45 . as_ref ()) . get () {
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
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_background_15 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_45 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (654311423f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (637534208f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_background_15 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
                 + {
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((0.9996999999999999f64 as f64) * (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: WindowItem :: FIELD_OFFSETS . default_font_size) . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
             + {
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (400f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
                 + {
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
             + {
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_margin }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
                 + {
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_placeholder_color }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_state }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_45 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (1090519039f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (1073741824f64 as u32)) as _ }
                        ) }
                     else {
                         (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_45 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (1090519039f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (1073741824f64 as u32)) as _ }
                        )) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
                 + {
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_45 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (1291867601f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (1291877119f64 as u32)) as _ }
                    ) . color ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
                 + {
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_45 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                    ) . color ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
                 + {
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_text_color }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_state }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_45 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (1090519039f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (1073741824f64 as u32)) as _ }
                        ) }
                     else {
                         (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_45 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                        )) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
                 + {
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_i_layout_16_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
                 + {
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_i_layout_16_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12__opacity_13_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_i_background_15_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_i_background_15_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_background_15 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_background_15 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
             + {
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
             + {
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_placeholder_10_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
             + {
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_i_text_input_11_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
             + {
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_margin }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
             + {
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_placeholder_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
             + {
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_root_clip_9_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
             + {
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_root_clip_9_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
             + {
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_y }
            ) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             InnerLineEditBase_root_8 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#i_base_17 }
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
                         * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_i_layout_16_layoutinfo_h }
                    ) . apply_pin (_self) . get ())) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (160f64 as sp :: Coord) . max (((({
                             * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_i_layout_16_layoutinfo_h }
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
                         * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_i_layout_16_layoutinfo_v }
                    ) . apply_pin (_self) . get ())) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (22f64 as sp :: Coord) . max (((({
                             * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_i_layout_16_layoutinfo_v }
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
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (((({
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_height }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ((({
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_width }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ({
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_rectangle_14_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_rectangle_14_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 2u32 => (({
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_i_background_15_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_i_background_15_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 3u32 => (({
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_i_layout_16_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_i_layout_16_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , ({
                     InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
                 + {
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 4u32 => (((({
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_height }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ((({
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_width }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ({
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12__opacity_13_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12__opacity_13_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 5u32 ..= 7u32 => return {
                     * & Self :: FIELD_OFFSETS . r#i_base_17 }
                 . apply_pin (_self) . item_geometry (index - 5u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 3u32 => {
                     * & Self :: FIELD_OFFSETS . r#i_base_17 }
                 . apply_pin (_self) . accessible_role (0) , 5u32 ..= 7u32 => {
                     * & Self :: FIELD_OFFSETS . r#i_base_17 }
                 . apply_pin (_self) . accessible_role (index - 5u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (3u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#i_base_17 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (5u32 ..= 7u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#i_base_17 }
                 . apply_pin (_self) . accessible_string_property (index - 5u32 + 1 , what) , _ => :: core :: default :: Default :: default () , }
             }
         # [allow (dead_code , unused)] pub fn r#fn_focus (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 . apply_pin (_self) . r#fn_focus ()) ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerButton_root_18 {
         r#root_18 : sp :: r#Empty , r#_opacity_19 : sp :: r#Opacity , r#rectangle_20 : sp :: r#BasicBorderRectangle , r#i_touch_area_43 : sp :: r#TouchArea , r#i_focus_scope_44 : sp :: r#FocusScope , r#root_18__opacity_19_dummy : sp :: Property < sp :: LogicalLength > , r#root_18_background : sp :: Property < slint :: Brush > , r#root_18_checkable : sp :: Property < bool > , r#root_18_checked : sp :: Property < bool > , r#root_18_height : sp :: Property < sp :: LogicalLength > , r#root_18_i_focus_scope_44_y : sp :: Property < sp :: LogicalLength > , r#root_18_i_layout_36_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_18_i_layout_36_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_18_i_layout_36_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_18_i_touch_area_43_x : sp :: Property < sp :: LogicalLength > , r#root_18_i_touch_area_43_y : sp :: Property < sp :: LogicalLength > , r#root_18_icon : sp :: Property < sp :: Image > , r#root_18_pressed : sp :: Property < bool > , r#root_18_rectangle_20_x : sp :: Property < sp :: LogicalLength > , r#root_18_rectangle_20_y : sp :: Property < sp :: LogicalLength > , r#root_18_state : sp :: Property < i32 > , r#root_18_text : sp :: Property < sp :: SharedString > , r#root_18_text_color : sp :: Property < slint :: Brush > , r#root_18_width : sp :: Property < sp :: LogicalLength > , r#root_18_x : sp :: Property < sp :: LogicalLength > , r#root_18_y : sp :: Property < sp :: LogicalLength > , r#root_18_clicked : sp :: Callback < () , () > , repeater0 : sp :: Repeater < InnerComponent__shadow_21 > , repeater1 : sp :: Repeater < InnerComponent__shadow_30 > , repeater2 : sp :: Repeater < InnerComponent__opacity_37 > , repeater3 : sp :: Repeater < InnerComponent__opacity_40 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerButton_root_18 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerButton_root_18 {
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
                         * & InnerButton_root_18 :: FIELD_OFFSETS . r#i_focus_scope_44 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ())) as bool)) as _ }
                 }
            ) ;
             _self . repeater2 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new ({
                         let r#tmp_root_18_icon = sp :: Image :: default () ;
                         ;
                         (((((r#tmp_root_18_icon . clone () . size ()) . r#width as f64) > (0f64 as f64))) && ((((r#tmp_root_18_icon . clone () . size ()) . r#height as f64) > (0f64 as f64)))) }
                     as bool)) as _ }
                 }
            ) ;
             _self . repeater3 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (((({
                         * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_text }
                    ) . apply_pin (_self) . get ()) != (sp :: SharedString :: from (""))) as bool)) as _ }
                 }
            ) ;
             sp :: Property :: link_two_way (({
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#i_touch_area_43 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , ({
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#i_focus_scope_44 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self)) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_background }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_18_state = ({
                             * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_root_18_state . clone () as f64) == (1f64 as f64)) {
                             slint :: Brush :: SolidColor (if InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_45 . as_ref ()) . get () {
                                 sp :: Color :: from_argb_encoded (2153865569f64 as u32) }
                             else {
                                 (sp :: Color :: from_argb_encoded (2164260863f64 as u32)) as _ }
                            ) }
                         else {
                             (if ((r#tmp_root_18_state . clone () as f64) == (2f64 as f64)) {
                                 if false {
                                     slint :: Brush :: SolidColor (if InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_45 . as_ref ()) . get () {
                                         sp :: Color :: from_argb_encoded (4280317678f64 as u32) }
                                     else {
                                         (sp :: Color :: from_argb_encoded (4278215658f64 as u32)) as _ }
                                    ) }
                                 else {
                                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_45 . as_ref ()) . get () {
                                         sp :: Color :: from_argb_encoded (4286216826f64 as u32) }
                                     else {
                                         (sp :: Color :: from_argb_encoded (4293980400f64 as u32)) as _ }
                                    )) as _ }
                                 }
                             else {
                                 (if false {
                                     slint :: Brush :: SolidColor (if InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_45 . as_ref ()) . get () {
                                         sp :: Color :: from_argb_encoded (4278212049f64 as u32) }
                                     else {
                                         (sp :: Color :: from_argb_encoded (4278221567f64 as u32)) as _ }
                                    ) }
                                 else {
                                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_45 . as_ref ()) . get () {
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
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_layout_36_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 2usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater2 . len () + _self . repeater3 . len ()) ;
                         InnerButton_root_18 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent__opacity_37 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater2 . instances_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         InnerButton_root_18 :: FIELD_OFFSETS . repeater3 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent__opacity_40 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
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
                                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_width }
                            ) . apply_pin (_self) . get () . get () as _ , r#spacing : 4f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_layout_36_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater2 . len () + _self . repeater3 . len ()) ;
                         InnerButton_root_18 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent__opacity_37 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater2 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         InnerButton_root_18 :: FIELD_OFFSETS . repeater3 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent__opacity_40 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
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
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_layout_36_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater2 . len () + _self . repeater3 . len ()) ;
                         InnerButton_root_18 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent__opacity_37 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater2 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         InnerButton_root_18 :: FIELD_OFFSETS . repeater3 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent__opacity_40 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
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
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_pressed }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         * & InnerButton_root_18 :: FIELD_OFFSETS . r#i_focus_scope_44 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ()) && (({
                         * & InnerButton_root_18 :: FIELD_OFFSETS . r#i_touch_area_43 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_rectangle_20_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (((({
                         * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_width }
                    ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_rectangle_20_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height }
                    ) . apply_pin (_self) . get () . get () as f64) - (((({
                         * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height }
                    ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ! ({
                         * & InnerButton_root_18 :: FIELD_OFFSETS . r#i_focus_scope_44 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (if ({
                             * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_pressed }
                        ) . apply_pin (_self) . get () {
                             2f64 }
                         else {
                             (if ({
                                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_checked }
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
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_text_color }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_18_state = ({
                             * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_root_18_state . clone () as f64) == (1f64 as f64)) {
                             slint :: Brush :: SolidColor (if InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_45 . as_ref ()) . get () {
                                 sp :: Color :: from_argb_encoded (1090519039f64 as u32) }
                             else {
                                 (sp :: Color :: from_argb_encoded (1073741824f64 as u32)) as _ }
                            ) }
                         else {
                             (if ((r#tmp_root_18_state . clone () as f64) == (3f64 as f64)) {
                                 slint :: Brush :: SolidColor (if InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_45 . as_ref ()) . get () {
                                     sp :: Color :: from_argb_encoded (4280317678f64 as u32) }
                                 else {
                                     (sp :: Color :: from_argb_encoded (4278215658f64 as u32)) as _ }
                                ) }
                             else {
                                 (if false {
                                     slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4293980400f64 as u32)) }
                                 else {
                                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_45 . as_ref ()) . get () {
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
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#_opacity_19 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((if ({
                         * & InnerButton_root_18 :: FIELD_OFFSETS . r#i_focus_scope_44 }
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
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#rectangle_20 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_45 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4282940159f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4284916195f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#rectangle_20 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#i_touch_area_43 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ({
                                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_checkable }
                            ) . apply_pin (_self) . get () {
                                 {
                                     ({
                                         * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_checked }
                                    ) . apply_pin (_self) . set (! ({
                                         * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_checked }
                                    ) . apply_pin (_self) . get () as _) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             ;
                             ({
                                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_clicked }
                            ) . apply_pin (_self) . call (& () . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#i_focus_scope_44 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#i_focus_scope_44 }
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
                                             * & InnerButton_root_18 :: FIELD_OFFSETS . r#i_touch_area_43 }
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
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18__opacity_19_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_checkable }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_focus_scope_44_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_touch_area_43_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_touch_area_43_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#rectangle_20 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#rectangle_20 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#rectangle_20 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#i_touch_area_43 }
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
                     InnerButton_root_18 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent__shadow_21 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1u32 => {
                     InnerButton_root_18 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent__shadow_30 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater1 . visit (order , visitor) }
                 2u32 => {
                     InnerButton_root_18 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent__opacity_37 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater2 . visit (order , visitor) }
                 3u32 => {
                     InnerButton_root_18 :: FIELD_OFFSETS . repeater3 . apply_pin (_self) . ensure_updated (|| InnerComponent__opacity_40 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
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
                         * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_layout_36_layoutinfo_h }
                    ) . apply_pin (_self) . get ())) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (20f64 as sp :: Coord) . max (((({
                             * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_layout_36_layoutinfo_h }
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
                         * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_layout_36_layoutinfo_v }
                    ) . apply_pin (_self) . get ())) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (20f64 as sp :: Coord) . max (((({
                             * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_layout_36_layoutinfo_v }
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
                     InnerButton_root_18 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent__shadow_21 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 => {
                     InnerButton_root_18 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent__shadow_30 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater1 . range ()) }
                 2u32 => {
                     InnerButton_root_18 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent__opacity_37 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater2 . range ()) }
                 3u32 => {
                     InnerButton_root_18 :: FIELD_OFFSETS . repeater3 . apply_pin (_self) . ensure_updated (|| InnerComponent__opacity_40 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater3 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_18 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent__shadow_21 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 => {
                     InnerButton_root_18 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent__shadow_30 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater1 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 2u32 => {
                     InnerButton_root_18 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent__opacity_37 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater2 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 3u32 => {
                     InnerButton_root_18 :: FIELD_OFFSETS . repeater3 . apply_pin (_self) . ensure_updated (|| InnerComponent__opacity_40 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
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
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (((({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ((({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_width }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_rectangle_20_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_rectangle_20_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 6u32 => (({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_touch_area_43_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_touch_area_43_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 7u32 => (({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord , ({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_focus_scope_44_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 8u32 => (((({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ((({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_width }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18__opacity_19_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18__opacity_19_dummy }
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
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_text }
                ) . apply_pin (_self) . get () , _ => :: core :: default :: Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent__shadow_21 {
         r#_shadow_21 : sp :: r#BoxShadow , r#rectangle_22 : sp :: r#BasicBorderRectangle , r#_shadow_23 : sp :: r#BoxShadow , r#rectangle_24 : sp :: r#BasicBorderRectangle , r#_shadow_25 : sp :: r#BoxShadow , r#rectangle_26 : sp :: r#BasicBorderRectangle , r#_opacity_27 : sp :: r#Opacity , r#rectangle_28 : sp :: r#BasicBorderRectangle , r#_shadow_21__opacity_27_dummy : sp :: Property < sp :: LogicalLength > , r#_shadow_21_dummy : sp :: Property < sp :: LogicalLength > , r#_shadow_21_rectangle_22_x : sp :: Property < sp :: LogicalLength > , r#_shadow_21_rectangle_22_y : sp :: Property < sp :: LogicalLength > , r#_shadow_21_rectangle_24_x : sp :: Property < sp :: LogicalLength > , r#_shadow_21_rectangle_24_y : sp :: Property < sp :: LogicalLength > , r#_shadow_21_rectangle_26_x : sp :: Property < sp :: LogicalLength > , r#_shadow_21_rectangle_26_y : sp :: Property < sp :: LogicalLength > , r#_shadow_21_rectangle_28_x : sp :: Property < sp :: LogicalLength > , r#_shadow_21_rectangle_28_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent__shadow_21 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_18 > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent__shadow_21 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (3f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded (1711276032f64 as u32)) as sp :: Color }
            ) ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0.5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_22 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_background) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_22 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_23 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_23 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_23 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded (637534208f64 as u32)) as sp :: Color }
            ) ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_23 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_24 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_background) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_24 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_25 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_25 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_25 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded (637534208f64 as u32)) as sp :: Color }
            ) ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_25 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0.5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_26 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_background) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_26 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_opacity_27 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (0.17f64) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_28 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                     color : sp :: Color :: from_argb_encoded (4294967295f64 as u32) , position : 1f64 as _ }
                 , sp :: GradientStop {
                     color : sp :: Color :: from_argb_encoded (16777215f64 as u32) , position : 0f64 as _ }
                ]))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_28 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21__opacity_27_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21_rectangle_22_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21_rectangle_22_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21_rectangle_24_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21_rectangle_24_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21_rectangle_26_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21_rectangle_26_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21_rectangle_28_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21_rectangle_28_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_22 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_22 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_22 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_23 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_23 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_23 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_23 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_23 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_24 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_24 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_24 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_25 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_25 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_25 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_25 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_25 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_26 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_26 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_26 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_28 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_28 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_28 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_28 }
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
                 0u32 => ((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21_rectangle_22_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21_rectangle_22_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => ((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 2u32 => ((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21_rectangle_24_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21_rectangle_24_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 3u32 => ((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21_rectangle_24_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21_rectangle_24_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 4u32 => ((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21_rectangle_26_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21_rectangle_26_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 5u32 => ((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21_rectangle_26_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21_rectangle_26_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 6u32 => ((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21_rectangle_28_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21_rectangle_28_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 7u32 => ((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21__opacity_27_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21__opacity_27_dummy }
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
     impl InnerComponent__shadow_21 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_18 >) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_18 > ;
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
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent__shadow_21 , sp :: ItemVTable , sp :: AllowPin > ;
             8usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_21 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_22 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_23 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_24 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_shadow_25 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_26 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#_opacity_27 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_21 :: FIELD_OFFSETS . r#rectangle_28 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent__shadow_21) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent__shadow_21 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent__shadow_21 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent__shadow_21 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent__shadow_21 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
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
     impl sp :: RepeatedItemTree for InnerComponent__shadow_21 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent__shadow_21 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent__shadow_30 {
         r#_shadow_30 : sp :: r#BoxShadow , r#rectangle_31 : sp :: r#BasicBorderRectangle , r#_opacity_32 : sp :: r#Opacity , r#_shadow_33 : sp :: r#BoxShadow , r#rectangle_34 : sp :: r#BasicBorderRectangle , r#_shadow_30__opacity_32_dummy : sp :: Property < sp :: LogicalLength > , r#_shadow_30_dummy : sp :: Property < sp :: LogicalLength > , r#_shadow_30_rectangle_31_x : sp :: Property < sp :: LogicalLength > , r#_shadow_30_rectangle_31_y : sp :: Property < sp :: LogicalLength > , r#_shadow_30_rectangle_34_x : sp :: Property < sp :: LogicalLength > , r#_shadow_30_rectangle_34_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent__shadow_30 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_18 > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent__shadow_30 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0.25f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded (1711276032f64 as u32)) as sp :: Color }
            ) ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0.25f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#rectangle_31 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_background) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#rectangle_31 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_opacity_32 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerButton_root_18 :: FIELD_OFFSETS . r#i_focus_scope_44 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () {
                         1f64 }
                     else {
                         (0.5f64) as _ }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_33 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_33 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_33 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded (637534208f64 as u32)) as sp :: Color }
            ) ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_33 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#rectangle_34 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_background) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#rectangle_34 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_45 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (352321535f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (335544320f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#rectangle_34 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#rectangle_34 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30__opacity_32_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30_rectangle_31_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30_rectangle_31_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30_rectangle_34_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30_rectangle_34_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#rectangle_31 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#rectangle_31 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#rectangle_31 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_33 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_33 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_33 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_33 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_33 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#rectangle_34 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#rectangle_34 }
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
                 0u32 => ((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30_rectangle_31_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30_rectangle_31_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => ((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 2u32 => ((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30_rectangle_34_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30_rectangle_34_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 3u32 => ((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30__opacity_32_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30__opacity_32_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 4u32 => ((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , (InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30__opacity_32_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30__opacity_32_dummy }
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
     impl InnerComponent__shadow_30 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_18 >) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_18 > ;
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
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent__shadow_30 , sp :: ItemVTable , sp :: AllowPin > ;
             5usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_30 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#rectangle_31 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_opacity_32 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#_shadow_33 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_30 :: FIELD_OFFSETS . r#rectangle_34 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent__shadow_30) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent__shadow_30 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent__shadow_30 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent__shadow_30 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent__shadow_30 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
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
     impl sp :: RepeatedItemTree for InnerComponent__shadow_30 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent__shadow_30 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent__opacity_37 {
         r#_opacity_37 : sp :: r#Opacity , r#image_38 : sp :: r#ImageItem , r#_opacity_37_dummy : sp :: Property < sp :: LogicalLength > , r#_opacity_37_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#_opacity_37_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent__opacity_37 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_18 > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent__opacity_37 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             sp :: Property :: link_two_way (({
                 * & InnerComponent__opacity_37 :: FIELD_OFFSETS . r#image_38 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) , (InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_icon) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ())) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_37 :: FIELD_OFFSETS . r#_opacity_37_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Item :: layout_info (({
                         * & InnerComponent__opacity_37 :: FIELD_OFFSETS . r#image_38 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_37 :: FIELD_OFFSETS . r#_opacity_37_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Item :: layout_info (({
                         * & InnerComponent__opacity_37 :: FIELD_OFFSETS . r#image_38 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_37 :: FIELD_OFFSETS . r#_opacity_37 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerButton_root_18 :: FIELD_OFFSETS . r#i_focus_scope_44 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () {
                         1f64 }
                     else {
                         (0.5f64) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_37 :: FIELD_OFFSETS . r#image_38 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if false {
                         (InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_text_color) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () }
                     else {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_37 :: FIELD_OFFSETS . r#image_38 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (4f64 as f64)) as f64) - (4f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__opacity_37 :: FIELD_OFFSETS . r#image_38 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             ({
                 * & InnerComponent__opacity_37 :: FIELD_OFFSETS . r#image_38 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (13f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__opacity_37 :: FIELD_OFFSETS . r#_opacity_37_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_37 :: FIELD_OFFSETS . r#image_38 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_37 :: FIELD_OFFSETS . r#image_38 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_37 :: FIELD_OFFSETS . r#image_38 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_37 :: FIELD_OFFSETS . r#image_38 }
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
                         * & InnerComponent__opacity_37 :: FIELD_OFFSETS . r#_opacity_37_layoutinfo_h }
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
                     * & InnerComponent__opacity_37 :: FIELD_OFFSETS . r#_opacity_37_layoutinfo_v }
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
                 0u32 => ((((((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (4f64 as f64)) as f64) - (4f64 as f64)) as sp :: Coord , 13f64 as sp :: Coord , {
                     let cache = (InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_layout_36_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                     * cache . get ((cache [0usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                 as sp :: Coord , 4f64 as sp :: Coord ,) , 1u32 => ((((((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (4f64 as f64)) as f64) - (4f64 as f64)) as sp :: Coord , 13f64 as sp :: Coord , ({
                     * & InnerComponent__opacity_37 :: FIELD_OFFSETS . r#_opacity_37_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__opacity_37 :: FIELD_OFFSETS . r#_opacity_37_dummy }
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
     impl InnerComponent__opacity_37 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_18 >) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_18 > ;
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
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent__opacity_37 , sp :: ItemVTable , sp :: AllowPin > ;
             2usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerComponent__opacity_37 :: FIELD_OFFSETS . r#_opacity_37 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__opacity_37 :: FIELD_OFFSETS . r#image_38 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent__opacity_37) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent__opacity_37 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent__opacity_37 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent__opacity_37 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent__opacity_37 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
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
     impl sp :: RepeatedItemTree for InnerComponent__opacity_37 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent__opacity_37 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent__opacity_40 {
         r#_opacity_40 : sp :: r#Opacity , r#text_41 : sp :: r#Text , r#_opacity_40_dummy : sp :: Property < sp :: LogicalLength > , r#_opacity_40_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#_opacity_40_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent__opacity_40 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_18 > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent__opacity_40 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#_opacity_40_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Item :: layout_info (({
                         * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#text_41 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#_opacity_40_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Item :: layout_info (({
                         * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#text_41 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#_opacity_40 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerButton_root_18 :: FIELD_OFFSETS . r#i_focus_scope_44 }
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
                     * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#text_41 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_text_color) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get ()) as _ }
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
                     * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#text_41 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((0.9996999999999999f64 as f64) * (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: WindowItem :: FIELD_OFFSETS . default_font_size) . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#text_41 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (400f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#text_41 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (4f64 as f64)) as f64) - (4f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#text_41 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#text_41 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_text) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#text_41 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#text_41 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let cache = (InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_layout_36_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                         * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#_opacity_40_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#text_41 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#text_41 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#text_41 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#text_41 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#text_41 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#text_41 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#text_41 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#text_41 }
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
                     * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#_opacity_40_layoutinfo_h }
                ) . apply_pin (_self) . get () , sp :: Orientation :: Vertical => ({
                     * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#_opacity_40_layoutinfo_v }
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
                 0u32 => ((((((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (4f64 as f64)) as f64) - (4f64 as f64)) as sp :: Coord , {
                     let cache = (InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_layout_36_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                     * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                 as sp :: Coord , {
                     let cache = (InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_layout_36_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                     * cache . get ((cache [2usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                 as sp :: Coord , 4f64 as sp :: Coord ,) , 1u32 => ((((((InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (4f64 as f64)) as f64) - (4f64 as f64)) as sp :: Coord , {
                     let cache = (InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_layout_36_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                     * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                 as sp :: Coord , ({
                     * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#_opacity_40_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#_opacity_40_dummy }
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
                 (1u32 , sp :: AccessibleStringProperty :: r#Label) => (InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_text) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () , _ => :: core :: default :: Default :: default () , }
             }
         }
     impl InnerComponent__opacity_40 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_18 >) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_18 > ;
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
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent__opacity_40 , sp :: ItemVTable , sp :: AllowPin > ;
             2usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#_opacity_40 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__opacity_40 :: FIELD_OFFSETS . r#text_41 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent__opacity_40) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent__opacity_40 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent__opacity_40 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent__opacity_40 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent__opacity_40 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
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
     impl sp :: RepeatedItemTree for InnerComponent__opacity_40 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent__opacity_40 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerAppWindow {
         r#root_1 : sp :: r#WindowItem , r#empty_3 : sp :: r#Empty , r#text_4 : sp :: r#Text , r#text_7 : sp :: r#Text , r#le_name_5 : InnerLineEdit_root_12 , r#button_6 : InnerButton_root_18 , r#root_1_empty_2_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_2_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_2_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_3_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_3_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_3_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_hello_response : sp :: Property < sp :: SharedString > , r#root_1_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_name : sp :: Property < sp :: SharedString > , r#root_1_x : sp :: Property < sp :: LogicalLength > , r#root_1_y : sp :: Property < sp :: LogicalLength > , r#root_1_send_hello : sp :: Callback < () , () > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerAppWindow >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , globals : Globals_AppWindow , window_adapter_ : sp :: OnceCell < sp :: WindowAdapterRc > , }
     impl InnerAppWindow {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerLineEdit_root_12 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#le_name_5 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 5u32 - 1 , tree_index_of_first_child + 6u32 - 1) ;
             InnerButton_root_18 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_6 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 2u32 - 1 , tree_index_of_first_child + 13u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_45 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4280821800f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                    ) . color ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
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
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
                                 + {
                                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_layout_36_layoutinfo_v }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (20f64 as sp :: Coord) . max (((({
                                         InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
                                     + {
                                         * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_layout_36_layoutinfo_v }
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
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as _ , r#spacing : 10f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
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
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
                             + {
                                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_layout_36_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (20f64 as sp :: Coord) . max (((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
                                 + {
                                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_layout_36_layoutinfo_h }
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
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
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
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
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
                                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
                             + {
                                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_layout_36_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (20f64 as sp :: Coord) . max (((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
                                 + {
                                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_layout_36_layoutinfo_v }
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
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
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
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
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
                                     InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
                                 + {
                                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_i_layout_16_layoutinfo_h }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (160f64 as sp :: Coord) . max (((({
                                         InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
                                     + {
                                         * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_i_layout_16_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
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
                         as _ , r#size : {
                             let r#tmp_empty_2_padding = 12f64 ;
                             ;
                             ((((({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                             + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                         as _ , r#spacing : 10f64 as _ , }
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
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
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
                                 InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
                             + {
                                 * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_i_layout_16_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (160f64 as sp :: Coord) . max (((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
                                 + {
                                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_i_layout_16_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
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
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
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
                                 InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
                             + {
                                 * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_i_layout_16_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (22f64 as sp :: Coord) . max (((({
                                     InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
                                 + {
                                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_i_layout_16_layoutinfo_v }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
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
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_hello_response }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("")) as sp :: SharedString }
            ) ;
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
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_name }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Slint Window")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_45 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                    ) . color ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_3_padding = 12f64 ;
                         ;
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                        ) . apply_pin (_self) . get () [1usize] as f64) - (r#tmp_empty_3_padding . clone () as f64)) as f64) - (r#tmp_empty_3_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Name:")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
                 + {
                     InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
                 + {
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_accepted }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_send_hello }
                            ) . apply_pin (_self) . call (& () . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
                 + {
                     InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
                 + {
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8_edited }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_name }
                            ) . apply_pin (_self) . set (args . 0 . clone () as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
                 + {
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_3_padding = 12f64 ;
                         ;
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                        ) . apply_pin (_self) . get () [1usize] as f64) - (r#tmp_empty_3_padding . clone () as f64)) as f64) - (r#tmp_empty_3_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
                 + {
                     InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
                 + {
                     * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_name }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
                 + {
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
                 + {
                     * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
             + {
                 * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
                 + {
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_send_hello }
                            ) . apply_pin (_self) . call (& () . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
                 + {
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Send Hello")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
                 + {
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_2_padding = 12f64 ;
                         ;
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
                 + {
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_45 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_45 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4278190080f64 as u32)) as _ }
                    ) . color ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                    ) . apply_pin (_self) . get () [5usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((((sp :: SharedString :: from ("Server says \"")) + (({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_hello_response }
                    ) . apply_pin (_self) . get () . as_str ()))) + (sp :: SharedString :: from ("\"") . as_str ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_2_padding = 12f64 ;
                         ;
                         ((((({
                             * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
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
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
             + {
                 * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12__opacity_13_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
             + {
                 * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_i_background_15_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
             + {
                 * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_i_background_15_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
             + {
                 * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18__opacity_19_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_checkable }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_focus_scope_44_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_touch_area_43_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_i_touch_area_43_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             InnerLineEdit_root_12 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#le_name_5 }
             . apply_pin (x)) ,) ;
             InnerButton_root_18 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_6 }
             . apply_pin (x)) ,) ;
             {
                 InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 . apply_pin (_self) . r#fn_focus () }
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
                 0u32 ..= 3u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_6 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 0u32 , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                ) . apply_pin (_self) . get () , sp :: Orientation :: Vertical => ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 3u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_6 }
                     . apply_pin (_self) . subtree_range (dyn_index - 0u32) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 3u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_6 }
                     . apply_pin (_self) . subtree_component (dyn_index - 0u32 , subtree_index , result) }
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
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , {
                     let r#tmp_empty_2_padding = 12f64 ;
                     ;
                     ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                 as sp :: Coord , 12f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 2u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , {
                     let r#tmp_empty_2_padding = 12f64 ;
                     ;
                     ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                 as sp :: Coord , 12f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 3u32 => (({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                ) . apply_pin (_self) . get () [5usize] as sp :: Coord , {
                     let r#tmp_empty_2_padding = 12f64 ;
                     ;
                     ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                 as sp :: Coord , 12f64 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord ,) , 4u32 => ({
                     let r#tmp_empty_3_padding = 12f64 ;
                     ;
                     ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as f64) - (r#tmp_empty_3_padding . clone () as f64)) as f64) - (r#tmp_empty_3_padding . clone () as f64)) }
                 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 5u32 => ({
                     let r#tmp_empty_3_padding = 12f64 ;
                     ;
                     ((((({
                         * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as f64) - (r#tmp_empty_3_padding . clone () as f64)) as f64) - (r#tmp_empty_3_padding . clone () as f64)) }
                 as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_empty_3_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 6u32 ..= 12u32 => return {
                     * & Self :: FIELD_OFFSETS . r#le_name_5 }
                 . apply_pin (_self) . item_geometry (index - 6u32 + 1) , 13u32 ..= 20u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_6 }
                 . apply_pin (_self) . item_geometry (index - 13u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 2u32 => sp :: r#AccessibleRole :: r#Button , 3u32 => sp :: r#AccessibleRole :: r#Text , 4u32 => sp :: r#AccessibleRole :: r#Text , 5u32 => {
                     * & Self :: FIELD_OFFSETS . r#le_name_5 }
                 . apply_pin (_self) . accessible_role (0) , 6u32 ..= 12u32 => {
                     * & Self :: FIELD_OFFSETS . r#le_name_5 }
                 . apply_pin (_self) . accessible_role (index - 6u32 + 1) , 2u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_6 }
                 . apply_pin (_self) . accessible_role (0) , 13u32 ..= 20u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_6 }
                 . apply_pin (_self) . accessible_role (index - 13u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (2u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
                 + {
                     * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18_text }
                ) . apply_pin (_self) . get () , (3u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (4u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("Name:") , (5u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#le_name_5 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (6u32 ..= 12u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#le_name_5 }
                 . apply_pin (_self) . accessible_string_property (index - 6u32 + 1 , what) , (2u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_6 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (13u32 ..= 20u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_6 }
                 . apply_pin (_self) . accessible_string_property (index - 13u32 + 1 , what) , _ => :: core :: default :: Default :: default () , }
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
             21usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 4u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 7u32 , children_index : 13u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 21u32 , parent_index : 0u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 6u32 , parent_index : 1u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 6u32 , parent_index : 1u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 9u32 , parent_index : 5u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 10u32 , parent_index : 5u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 10u32 , parent_index : 5u32 , item_array_index : 8u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 10u32 , parent_index : 6u32 , item_array_index : 9u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 11u32 , parent_index : 8u32 , item_array_index : 10u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 13u32 , parent_index : 10u32 , item_array_index : 11u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 13u32 , parent_index : 10u32 , item_array_index : 12u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 20u32 , parent_index : 2u32 , item_array_index : 13u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 2u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 1u32 , parent_index : 2u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 2u32 , parent_index : 2u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 3u32 , parent_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 21u32 , parent_index : 2u32 , item_array_index : 14u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 21u32 , parent_index : 2u32 , item_array_index : 15u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 21u32 , parent_index : 13u32 , item_array_index : 16u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerAppWindow , sp :: ItemVTable , sp :: AllowPin > ;
             17usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#empty_3 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#root_18 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_7 }
            ) , sp :: VOffset :: new ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#text_4 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
             + {
                 * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#root_12 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
             + {
                 * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#_opacity_13 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
             + {
                 * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_background_15 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
             + {
                 InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
             + {
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_8 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
             + {
                 * & InnerLineEdit_root_12 :: FIELD_OFFSETS . r#rectangle_14 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
             + {
                 InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
             + {
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#root_clip_9 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
             + {
                 InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
             + {
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_placeholder_10 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#le_name_5 }
             + {
                 InnerLineEdit_root_12 :: FIELD_OFFSETS . r#i_base_17 }
             + {
                 * & InnerLineEditBase_root_8 :: FIELD_OFFSETS . r#i_text_input_11 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#_opacity_19 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#i_touch_area_43 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#i_focus_scope_44 }
            ) , sp :: VOffset :: new ({
                 InnerAppWindow :: FIELD_OFFSETS . r#button_6 }
             + {
                 * & InnerButton_root_18 :: FIELD_OFFSETS . r#rectangle_20 }
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
             inner . globals . global_ColorSchemeSelector_45 . clone () . init (& inner) ;
             inner . globals . global_CupertinoPalette_46 . clone () . init (& inner) ;
             inner . globals . global_StyleMetrics_47 . clone () . init (& inner) ;
             InnerAppWindow :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             core :: result :: Result :: Ok (Self (inner)) }
         # [allow (dead_code)] pub fn get_hello_response (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_hello_response }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_hello_response (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_hello_response }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_name (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_name }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_name (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_name }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_send_hello (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_send_hello }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_send_hello (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAppWindow :: FIELD_OFFSETS . r#root_1_send_hello }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
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
         global_ColorSchemeSelector_45 : :: core :: pin :: Pin < sp :: Rc < InnerColorSchemeSelector_45 >> , global_CupertinoPalette_46 : :: core :: pin :: Pin < sp :: Rc < InnerCupertinoPalette_46 >> , global_StyleMetrics_47 : :: core :: pin :: Pin < sp :: Rc < InnerStyleMetrics_47 >> , }
     impl :: core :: default :: Default for Globals_AppWindow {
         fn default () -> Self {
             Self {
                 global_ColorSchemeSelector_45 : InnerColorSchemeSelector_45 :: new () , global_CupertinoPalette_46 : InnerCupertinoPalette_46 :: new () , global_StyleMetrics_47 : InnerStyleMetrics_47 :: new () , }
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
