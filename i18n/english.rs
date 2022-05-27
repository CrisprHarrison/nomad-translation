// Comments and entries are sync from the english version, it's not
// possible to have language specific comments at the moment.
// You can use this entry to make a specific comment.
language_note ""

// comments with "ICON FIT" should be short, ideally < 10 characters

// When in doubt, leave an empty string, it will fallback to english
// Some terms should probably be left untranslated
// For sure: Voxel, Matcap, DynTopo, PBR, Dyntopo
// Not sure: Roughness/Metalness? Mesh? Sub? tool names? etc

// ----------------------------------------------
// general stuffs

// Popup question, confirm? [yes/cancel] [ok/cancel] [delete/cancel]
confirm "Confirm?"
yes "Yes"
ok "Ok"
delete "Delete"
cancel "Cancel"

// feature: Auto / Off / On
on "On"
off "Off"
auto "Auto"

// coordinate
X "X"
Y "Y"
Z "Z"

// general
advancedSettings "Advanced"
notSaved "These options are not saved in the settings."

// generic warning when there is no mesh selected
noSelectedMesh "No selected mesh."

// generic warning when only one mesh needs to be selected
multipleObjectWarning "Multiple meshes are selected, please select only one mesh."

// ----------------------------------------------

// when you launch the app and there is missing Nomad/data files
loading.reprocess "Missing thumbnails, reprocessing files... ($0/$1)

$2"

// main pbr channel
baseColor "Color"
roughness "Roughness"
metalness "Metalness"

// ----------------------------------------------
// about
about.minify "Minify UI"
about.minify.help "You can also tap your screen with 4 fingers, if your device supports it."
about.turntable "Turntable"
about.turntableSpeed "Turntable Speed"
about.credits "Credits"
about.creditsOpenSource "Open-Source"
about.creditsArts "Matcaps & Hdris"
about.languages "Languages"
about.languages.help "The translation file is available at github.com/stephomi/nomad-translation"
about.openUrl "Open?"
// nomad
about.website "Website"
about.forum "Forum"
about.manual "Manual"
about.mail "Support"
// social
about.twitter "Twitter"
about.instagram "Instagram"
about.facebook "Facebook"
about.discord "Discord"

// ----------------------------------------------
// alert
alert.hole.nothing "The object has no holes!"
alert.shape.notVisible "The current mesh is invisible!"
alert.trim.nothing "Nothing to trim."
alert.trim.full  "Abort trim: the mesh is fully trimmed."
alert.mask.noExtract "Nothing to extract!"
alert.mask.noSplit "Nothing to split!"
alert.view.disabled "Features disabled in View Mode:"
alert.view.disabled.widgetPrimitive "Primitive widgets"
alert.separate.fail "Could not separate: the object has only one part!"
alert.voxelRemesh.success "Remeshed!"
alert.voxelRemesh.empty "Abort remesh: result mesh has no faces."
alert.voxelRemesh.invalidInput "Invalid input!"
alert.matrix.clone "The object will be duplicated"
alert.gizmo.usePivot "Use custom pivot."
alert.gizmo.useAuto "Use automatic pivot."
alert.gizmo.editPivot "Edit pivot mode."
alert.gizmo.editObject "Edit object mode."
alert.dynamic.enable "Active dynamic topology"
alert.dynamic.disable "Disable dynamic topology"
alert.colorPicker "Drag your finger on the mesh to pick a color."
alert.backgroundTransform "Simple tap to exit transform mode."
alert.camera.resetView "Reset view"
alert.camera.snapView "Snap view"
alert.mask.show "Show mask"
alert.mask.hide "Hide mask"
alert.selection.lock "Lock selection"
alert.selection.unlock "Unlock selection"
alert.selection.isolate "Isolate selection"
alert.selection.showAll "Show all"
alert.quickSave "Saving..."
alert.forceShowPainting.fill "Show painting activated, [Paint all] was used."
alert.forceShowPainting.tool "Show painting activated, the object was painted."
alert.multiresLost "Multiresolution will be lost!"
alert.rangeWarning "The detail value is high and might require lot of memory!"
// autosave popup
alert.autoSave.auto "Autosave in... $0s"
// bottom warning
alert.warning.needLayer "The current tool requires an active layer."
alert.warning.paintingHidden "Painting hidden: show it again in Settings panel."
alert.warning.noPartialWireframe "Partial drawing is disabled when wireframe is displayed."
// bottom tip
alert.tip.shapeOrthographic "Consider using orthographic camera if you want to avoid perspective frustum distortion when using screen projector."
// undo
alert.state.trial "Trial version: undo canceled"

// ----------------------------------------------
// background
background "Background"
background.color "Color"
background.environment "Environment"
background.blur "Blur"
background.exposure "Exposure"

background.imageEnable "Reference image"
background.imageOverlay "Overlay"
background.imageAlpha "Alpha"
background.imageReset "Reset settings"
background.imageTransform "Transform"
// transform
background.imageX "Position X"
background.imageY "Position Y"
background.imageRotation "Rotation"
background.imageScale "Scale"

// ----------------------------------------------
// camera
camera "Camera"
// saved views
camera.updateView "Update view point?"
camera.addView "Add View"
camera.focusOn "Focus on"
// projection
camera.projection "Projection"
camera.orthographic "Orthographic"
camera.perspective "Perspective"
camera.fov "Vertical Fov"
camera.focal "focal $0mm (35mm sensor)"
// orbit
camera.orbit "Orbit mode"
camera.orbit.help "Trackball gives more degree of freedom you can also roll the camera with 2 fingers."
camera.trackball "Trackball"
camera.turntable "Turntable"
// speed
camera.speed "Speed"
camera.rotation "Rotation"
camera.panning "Panning"
camera.zooming "Zooming"
// misc
camera.resetView "Reset view"
camera.snapView "Snap view"
// interaction
camera.pivot "Pivot"
camera.doubleTapMesh "Double tap on mesh"
camera.doubleTapBackground "Double tap on background"
camera.doubleTapPivot "Update on double tap"
camera.doubleTapPivot.help "Update the rotation pivot when double tapping on the object surface."
camera.airPivot "Air pivot"
camera.airPivot.help "Allow new pivot even when zooming outside the object surface."
camera.autoPivot "On camera gesture start"
camera.autoPivot.help "Update the pivot when you start interacting with the camera."
camera.doubleTapFocus "Focus"
camera.doubleTapFocus.help "When double tapping on the mesh, the camera will pan and focus on the picked point."
camera.doubleTapFocusSelection "Focus on selection"
camera.doubleTapFocusSelection.help "When double taping on background focus on the selected mesh instead of the whole scene."

// toolbox context, only a few tools are display in some cases
// (only visible in expanded toolbox mode)
context.multiselection "Multiselection"
context.triplanar "Triplanar"
context.primitive "Primitive"

// scene and layer lists
curve.preset "Preset"
curve.custom "Custom"

// ----------------------------------------------
// debug
debug.uvPrimitive.warning "Disable this option if you don't need UVs (extra memory)."
debug.uvPrimitive "Keep primitive UVs"
debug.uvPrimitive.help "For now, only Box and Sphere are supported.

Other types will be supported in the future."
debug.uvNormalize "Normalize UVs"
debug.uvNormalize.help "Nomad will normalise the UVs inside the [0-1] tile."
debug.uvBFF "Add BFF UVs"
debug.uvBFF.help "Add an alternative unwrapping method (boundary first flattening).

Note that BFF will produce overlaps if your mesh topology is different than a disk or a sphere."
debug.logs "Logs"
debug.heightmap "Heightmap"
debug.graphics "Graphics"
debug.thumbnails "Make store thumbnails"

// scene and layer lists
expandList "UI: Expand list"
expandList.help "Just an UI option for easier list management."

// ----------------------------------------------
// file
file.project.empty "You have no saved project yet!"
file.project.unsaved "Unsaved changes!"
file.project.loseUnsaved "You will lose unsaved changes!"
file.project.lastManualSave "Preview of the last manual save"
file.project.trialNoOpen "Trial version: You won't be able to re-open the current project!"
file.project.trialOnlyOpen "Trial version: you can only re-open your current project!"

// ----------------------------------------------
// project
file.project "Project"
file.project.save "Save"
file.project.save.confirm "Save $0?"
file.project.saveAs "Save As"
file.project.saveAs.confirm "Overwrite $0?"
file.project.open "Open"
file.project.open.confirm "Open $0?"
file.project.add "Add to scene"
file.project.add.confirm "Add $0 to the scene?"
file.project.new "New"
file.project.new.confirm "Create new scene?"
file.project.delete "Delete"
file.project.delete.confirm "Delete $0?"
file.project.delete.confirmActive "Delete $0?

This is the current active project!"
file.project.delete.confirmOk "Are you sure?"
file.project.rename "Rename"

// autosave
file.project.autoSave "Project Auto Save"
file.project.autoSave.confirm "Disable Auto Save?"
file.project.autoSave.help "Save your project in a separate file at regular interval.
The autoSave file can be found in:

$0"
file.project.autoSave.popup "Popup timeout"
file.project.autoSave.minutes "Timer popup"
file.project.autoSave.delete "Discard Auto Save"
file.project.autoSave.delete.confirm "Confirm?"

file.importSettings "Import options"

// import
file.import.title "Import"
file.import.title.help "Supported format:
- Wavefront (.obj)
- glTF 2.0 (.glb .gltf)
- STL (.stl)"
file.importOpen "Open"
file.importOpen.confirm "Import new file?"
file.import.add "Add to scene"
file.import.add.confirm "Import new file?"

file.exportSelection "Export selection only"
file.exportSelection.help "Export the current selected mesh instead of the entire scene."
file.convertToQuad "Reconstruct quad"
file.convertToQuad.help "Reconstruct quads from triangles by pairing triangle (if they are adjacent in the files)."

// export
file.export.title "Export"
file.export.title.help "If possible favor glTF export as it supports more features than other formats."

// generic export
file.export.texture "Export textures"
file.export.texture.help "This option doesn't bake vertex colors into textures.

It only re-export textures in case they were already present in an imported file."
file.export.normal "Export normals"
file.export.normal.help "Check this option if you want to open the file in another softwares.

Nomad always ignore the normals as it will recompute them."

// gltf
file.export.gltf "Export glTF 2.0"
file.export.gltfLayer "Export layers"
file.export.gltfLayer.help "Export layers as morphs. Officially supported by glTF so it should work on other softwares as well."
file.export.gltfLayerPaint "Export layer paint"
file.export.gltfLayerPaint.help "Export layer painting. It is usually ignored by other softwares."
file.export.gltfLayerNomad "Nomad layer encoding"
file.export.gltfLayerNomad.help "Can make the file smaller by writing paint layer in 8-bit instead of 16-bit.

This encoding only works in Nomad as it will use absolute painting.
Other softwares will have incorrect result as glTF requires relative encoding."
file.export.gltfColor0 "Export vertex colors"
file.export.gltfColor0.help "Export vertex colors. Officially supported by glTF so it should work on other softwares as well."
file.export.gltfColor1 "Export pbr paint"
file.export.gltfColor1.help "Export roughness, metalness and mask painting. This will be ignored by other softwares."

// obj
file.export.obj "Export OBJ"
file.export.objWarning "Layers and extra painting (roughness metalness and mask) will be lost."
file.export.objColorAppend "Export vertex colors"
file.export.objColorAppend.help "Append color information after vertices.

Some 3d softwares will be able to read it but not all of them."

// stl
file.export.stl "Export STL"
file.export.stlWarning "Layers and extra painting (roughness metalness and mask) will be lost."
file.export.stlColor "Export vertex colors"
file.export.stlColor.help "Some 3d softwares will be able to read it but not all of them."
file.export.stlAscii "By default the format is binary.

You can choose to export to text format (ASCII) but the file will be bigger."

file.settings.title "Settings"
file.settings.title.help "Most of the app settings are saved here (Camera, Interface etc).

Some resources are saved separately and automatically, it includes:
- HDRs
- Matcaps
- Alphas
- Textures
- Backgrounds
- Projects

At the moment brush settings cannot be saved but custom brush managament is planned."

// settings
file.settings.reset "Reset to default"
file.settings.reset.confirm "Reset preference settings?

The current scene and lighting will be kept intact.

Also these resources are saved separately and won't be impacted:
- File projects
- Matcaps
- Environments
- Textures
- Backgrounds
- Tool presets
- Material presets"

// render
file.render "Render"
file.render.showInterface "Show interface"
file.render.renderRatio "Render ratio"
file.render.renderRatio.help "A value of 1.0 means that Nomad will render at the same resolution as the requested image size below.

Use this option if you cannot render at certain resolution (crashes because of lack of memory)."
file.render.help "Render ratio"
file.render.size "Final size"
file.render.size.custom "Custom"
file.render.screenResolution "screen"
file.render.export "Export png"
file.render.width "Width"
file.render.height "Height"
file.render.warn "The export resolution is high ($0x$1)!

Make sure to save your project in case your device runs out of VRAM and crashes."
file.render.transparent "Transparent background"
file.render.transparent.help "This option can be useful if you want to insert the mesh in a 2d creation software.

Partial object transparency is not supported for now."

// ----------------------------------------------
// gesture menu
gesture.useGlobal "Use global settings"
gesture.useGlobal.help "By default, the tools share the same pressure settings.

Uncheck this option if you want specific pressure settings for this tool."

gesture.pressure "Pressure"
gesture.pressureTitle "Pressure ($0)"
gesture.pressure.noTool "This tool doesn't use pen pressure."
gesture.pressure.noGrab "Grab stroke type will ignore pressure settings."
gesture.pressure.radius "Radius"
gesture.pressure.intensity "Intensity"
gesture.pressure.useRadius "Active"
gesture.pressure.useIntensity "Active"
gesture.pressure.curveRadius "Radius"
gesture.pressure.curveIntensity "Intensity"

gesture.cameraInteraction "Camera:"
gesture.sculptInteraction "Sculpt:"
gesture.interaction.fingerAndStylus "Finger and Stylus"
gesture.interaction.finger "Finger"
gesture.interaction.stylus "Stylus"

gesture.fingerLighting "Rotate lighting (3 fingers)"
gesture.fingerLighting.help "Drag 3 fingers horizontally on the canvas to rotate the environment, lights and matcap."
gesture.fingerRadius "Edit tool radius (3 fingers)"
gesture.fingerRadius.help "Drag 3 fingers vertically to edit the radius."

gesture.fingerSmooth "Finger always smooths"
gesture.unknownPressure "Allow unrecognized pressure"
gesture.unknownPressure.help "Check this option if the pressure doesn't work with your pencil or if you need pressure finger."

// pencil
gesture.pencilAction.none "None"
gesture.pencilAction.smooth "Smooth"
gesture.pencilAction.alt "Add/Sub"
gesture.pencilAction.android "Pencil button"
gesture.pencilAction.android.help "Experimental"
gesture.pencilAction.ios "Pencil double tap"
gesture.pencilAction.ios.help "Only active for Apple Pencil 2nd gen."

// history
gesture.history "History shortcuts"
gesture.history.help "- Undo: tap with 2 fingers
- Redo: tap with 3 fingers
- Undo/Redo: hold 2/3 fingers down (continuous)"

// size rejection
gesture.useSizeRejection "Use size rejection"
gesture.useSizeRejectionConfirm "Make sure to disable this option if you have trouble interacting with the canvas!"
gesture.useSizeRejection.help "Reject input if the contact area size larger than this value.

Might not work on every device."
gesture.sizeRejection "Max size threshold"
// help
gesture.interaction.title "Gesture"
gesture.interaction.title.help "These options are always global."

// ----------------------------------------------
// history
history "History"
history.root "Root"
history.undoConfirm "Do you confirm undoing all these operations?"
history.undoWarning "If you make an edit afterwards you might lose lot of changes."
history.stack "Stack"
history.limitSize "History limit (Mb)"
history.limitSize.help "Maximum size (in Mb) of the history.

The history will be updated on the next recorded operation."
history.limitStack "Stack limit"
history.limitStack.help "Maximum number of operation the application can keep.

The history will be updated on the next recorded operation."
history.rangeProtect "Range protection"
history.rangeProtect.help "If you go far in the history, it will prompt a confirm dialog before undoing many operations."
history.restoreCamera "Restore camera"
history.restoreCamera.help "Enable this option to restore the saved camera viewpoint when you undo/redo an action."
// display undo/redo
history.state.undo "Undo: $0"
history.state.redo "Redo: $0"
history.state.symmetrySplit "Symmetry Split"
history.state.voxelRemesh "Voxel remesh"
history.state.surfaceRemesh "Surface remesh"
// state multires
history.state.multiresToDynamic "Multires to Dynamic"
history.state.multiresLevel "Resolution change"
history.state.multiresSubdivide "Subdivide"
history.state.multiresReverse "Reversion"
history.state.multiresDeleteLower "Delete lower"
history.state.multiresDeleteHigher "Delete higher"
// mesh
history.state.meshDynamicToStatic "Dynamic to Static"
history.state.meshStaticToDynamic "Static to Dynamic"
history.state.meshSymmetryUpdate "Symmetry update"
history.state.meshMatrixUpdate "Matrix update"
history.state.meshVisibility "Visibility"
history.state.meshMaterial "Material change"
// state scene
history.state.sceneAddRemove "Scene"
history.state.sceneMeshOrder "Mesh order"
// state layer
history.state.layerOrder "Move layer order $0"
history.state.layerMergeRedo "Unmerge layer $0"
history.state.layerCreate "Create layer $0"
history.state.layerDelete "Delete layer $0"
history.state.layerMerge "Merge layer $0"
history.state.layerHide "Hide layer $0"
history.state.layerShow "Show layer $0"
history.state.layerSelect "Select layer $0"
history.state.layerUnselect "Unselect layer $0"
history.state.layerName "Layer $0 name"
history.state.layerFactor "Layer $0 factor"
history.state.layerFactorOffset "Layer $0 offset factor"
history.state.layerFactorColor "Layer $0 color factor"
history.state.layerFactorRoughness "Layer $0 roughness factor"
history.state.layerFactorMetalness "Layer $0 metalness factor"
// state light
history.state.lightVisible "light $0 visible"
history.state.lightIntensity "Light $0 intensity"
history.state.lightColor "Light $0 color"
history.state.lightPosition "Light $0 position"
history.state.lightShadow "Light $0 shadow"
history.state.lightShadowType "Light $0 shadow type"
history.state.lightShadowBias "Light $0 shadow bias"
history.state.lightShadowSoftness "Light $0 shadow softness"
history.state.lightAttachment "Attachment light $0"
history.state.lightAdd "Add light $0"
history.state.lightDelete "Delete light $0"
history.state.lightCopy "Copy light $0"
history.state.lightMove "Move light $0"
history.state.lightType "Light $0 type"
history.state.lightSpotAngle "Light $0 spot angle"
history.state.lightSpotSoftness "Light $0 spot softness"
// state view
history.state.viewAdd "Add view $0"
history.state.viewMove "Move view $0"
history.state.viewDelete "Delete view $0"

// ----------------------------------------------
// interface
interface "Interface"

// bottom buttons
interface.bottomButtons "Add shortcuts (bottom)..."

// colors
interface.colors "Main colors"
interface.colorSelect "Color widget"
interface.colorBase "Color base"
interface.colorBaseTransparent "Color panel"
interface.panelTransparent "Transparent panel"
interface.blurFactor "Blur strength"

// color preset
interface.colorsPresets "Color presets"
interface.presetBlurRed "Red"
interface.presetBlurBlue "Blue"
interface.presetBlurGreen "Green"
interface.presetBlurYellow "Yellow"
interface.presetBlackWhite "Black & White"
interface.presetWhiteBlack "White & Black"
interface.presetLividOrange "Livid & Orange"
interface.presetCardboard "Cardboard"
interface.presetDefault "Default"

// style
interface.style "Style"
interface.resetAll "Reset Interface"
interface.resetAll.confirm "Reset interface settings?"
interface.flipTop "flip top"
interface.flipBottom "flip bottom"
interface.flipMiddle "flip middle"
interface.showTooltips "Show tooltips"
interface.showTooltips.help "This is a tooltip."
interface.materialPreview "Material Picker preview"
interface.toolboxHide "Auto-hide toolbox"
interface.toolboxHide.help "Enable this option if you want to hide the toolbox."
interface.toolboxMaxColumn "Max column toolbox"
interface.toolboxResetOrder "Reset toolbox order"
interface.curveToolSymmetric "Symmetric tool curve widget"
interface.curveToolSymmetric.help "The widget can be found in the Tool panel under the falloff option."
interface.scale "Overall scale"
interface.cursorStep "Vertical spacing"
interface.panelWidth "Panel width"
interface.fontScale "Font scale"

// ----------------------------------------------
// layer sub menu
layer.action "Action"
layer.name "Name"
layer.delete "Delete"
layer.move "Move"
layer.duplicate "Duplicate"
layer.mergeDown "Merge down"
layer.factors "Channel factors"
layer.offsetFactor "Position"
layer.colorFactor "Color"

// ----------------------------------------------
// layers menu
layers.addLayer "Add layer"
layers.title "Layers"
layers.title.help "Layers can record position offsets and painting, it can be useful for non-linear workflow.
For example by experimenting different facial expression without relying on the history stack to undo the changes.

For painting data, layers are sorted in a top-down fashion; so layers on top will mask the lower ones.

In order to resolve the layer opacity, all painting data (color, roughness, metalness) share the same mask.
You can reset part of this mask (and thus, the layer influence) by using the 'DelLayer' tool."
layers.primitive "Layers are unavailable for primitives."
layers.baseSelected "None"

// ----------------------------------------------
// light sub menu
light "Light"
light.intensity "Intensity"
light.attachment "Attachment"
light.attachment.fixed "Fixed"
light.attachment.camera "Camera"
light.attachment.environment "Environment"
light.attachment.help "-- Fixed
Light orientation won't change.

-- Camera
Light orientation depends on the camera view."
light.type "Type"
light.type.directional "Directional"
light.type.spot "Spot"
light.type.point "Point"
light.spotAngle "Cone angle"
light.spotSoftness "Softness"
light.shadowCast "Shadow"
light.shadowType.shadowMap "Shadow map"
light.shadowType.screenspace "Screenspace"
light.shadowType.screenspace.help "Experimental feature, it might be removed in a future release.

This option is meant to be used with the softness shadow feature."
light.shadowBias "Bias"
light.shadowSoftness "Softness"
light.contactShadow "Contact"
light.contactShadow.help "If auto is selected, only the most dominant light will have contact shadow."
light.visible "Show"
light.resetPosition "Recenter"

// ----------------------------------------------
// material
material "Material"
material.addNew "Add new"
// if the shading mode is in matcap or unlit
material.pbrRoughnessMetalnessWarning "Roughness and Metalness requires PBR shading mode."
material.pbrReflectanceWarning "Reflectance requires PBR shading mode."
material.pbrRefractionWarning "Refraction requires PBR shading mode."
material.pbrSubsurfaceWarning "Subsurface requires PBR shading mode."
// refraction
material.ior "Index of Refraction"
material.paintingOverride "Override painting"
material.paintingOverride.help "There is two roughness at play, the one that drives the surface and the one in the inside.

However there is only one paintable roughness, so the two roughness have the same values.

You can use this slider to override the surface roughness to make more glossy."
material.refractionSurfaceGlossiness "Surface glossiness"
material.refractionSurfaceGlossiness.help "- at 0, the surface is using the painted roughness
- at 1, the surface is completely smooth"
material.refractionInteriorRoughness "Interior roughness"
material.refractionInteriorRoughness.help "- at 0, the interior is using the painted roughness
- at 1, the interior is completely rough"
material.paintGlossy "Paint glossy"
material.paintGlossy.help "It will paint the object with a roughness and metalness of 0, thus allowing sharp refraction.

This is the same as going in the painting menu and using the paint all feature with color and metalness disabled."
// absorption
material.absorptionEnable "Absorption"
material.absorptionEnable.help "Simulate the light being absorbed when it travels through the volume.

Thin parts will bright as it lets more light pass through, while thick areas will be darker.

The effect heavily depends on the mesh geometry, only an approximation of the mesh thickness is used."
material.absorptionFactor "Factor"
// subsurface
material.subsurfaceDepth "Depth"
material.translucency "Translucency"
material.translucency.help "You need to have a light that casts shadows in order to see translucency."
// type
material.opacity "Opacity"
material.type.opaque "Opaque"
material.type.subsurface "Subsurface"
material.type.subsurface.help "To get the best result, you can switch to PBR shading mode and use at least one directional light, ideally with a dim environment."
material.type.blending "Blending"
material.type.blending.help "Make the mesh semi-transparent by tweaking the opacity value.

Note that because of real-time constraints, you can have noticeable visual artifacts if your object has a complex shape."
material.type.additive "Additive"
material.type.additive.help "Make the mesh semi-transparent by tweaking the opacity value.

This method tends to have less artifacts than the Blending method, but the object will be brighter."
material.type.dithering "Dithering"
material.type.dithering.help "Make the object semi-transparent by discarding some pixels in a random fashion."
material.type.refraction "Refraction"
material.type.refraction.help "This mode can be used to simulate glass material.

Because of real time constraints, self-refraction or multi-layered refraction is limited."
// shadows
material.castShadows "Cast shadows"
material.receiveShadows "Receive shadows"
// backface
material.twoSided "Two sided"
material.alwaysUnlit "Always unlit"
material.flipCulling "Inverse culling"
// material
material.reflectance "Reflectance"
material.reflectance.help "Control the amount of reflection the material will receive for non-metallic materials.

Most of the time, the default value should be used (0.5, which corresponds to the standard 4% reflected light at normal angle)."

// ----------------------------------------------
// menu name (visible on small screen menu are collapsed)
menu.files "Files"
menu.scene "Scene"
menu.multires "Multires"
menu.voxel "Voxel"
menu.dyntopo "Dyntopo"
menu.topology "Deci/UV..."
menu.primitive "Primitive"
menu.render "Render"
menu.material "Material"
menu.postProcess "PostProcess"
menu.camera "Camera"
menu.background "Background"
menu.tool "Tool"
menu.stroke "Stroke"
menu.paint "Paint"
menu.symmetry "Symmetry"
menu.pressure "Pressure"
menu.layers "Layers"
menu.settings "Settings"
menu.interface "Interface"
menu.history "History"
menu.historySettings "Settings"
menu.about "About"
menu.debug "Debug"

// ----------------------------------------------
// mesh sub menu
mesh.action "Action"
mesh.holeClose "Close holes"
mesh.holeDetail "Detail"
mesh.separate "Separate"
mesh.triplanarWarning "Layers, painting and multiresolution will be lost."
mesh.triplanarResolution "Resolution"
mesh.triplanarCubic "Force cubic"
mesh.triplanarConvert "Convert"
mesh.name "Name"
mesh.type "Type"
mesh.typeStatic "Static"
mesh.typeMultiresolution "Multiresolution"
mesh.typeDynamic "Dynamic"

// ----------------------------------------------
// painting
paint.useGlobal "Global material"
paint.useGlobal.help "If this option is enabled, the selected material will be the same as the other tools.

Note that it only takes into account roughness, metalness and color settings."
paint.usePainting "Stroke painting"
paint.intensity "Paint intensity"
paint.paintAll "Paint all"
paint.paintAll.help "Apply the current material to the mesh."
paint.paintAllForce "Force paint all"
paint.paintAllForce.help "Apply the current material to the mesh.

Masked area and disabled channels won't be painted."
paint.strokePaintingTitle "Painting ($0)"
paint.layerWarning "Channel masking will be ignored if you try to apply it on a layer."
paint.texture.title "Texture"
paint.texture.title.help "An image that will color your brush stroke.

Note that it will share the alpha's tiling and scale settings."
paint.texture.warningEnable "Stroke painting needs to be enabled to allow texture projection (checkbox on top)!"
paint.texture.warningIgnored "The current tool cannot use textures!"
paint.useAlpha "Use stroke alpha"
paint.useAlpha.help "Using the alpha set in the stroke menu to modulate the painting."
paint.useFalloff "Use stroke falloff"
paint.useFalloff.help "Using the falloff set in the stroke menu to modulate the painting."

// ----------------------------------------------
// popup (for example tap on a tool, to open edit popup)
popup.save "Save"
popup.save.confirm "Confirm save?"
popup.lastSave "Last save"
popup.lastSave.confirm "Load last save?"
popup.reset "Reset"
popup.reset.confirm "Confirm reset?"
popup.clone "Clone"
popup.rename "Rename"
popup.delete "Remove"
popup.delete.confirm "Confirm deletion?"
popup.delete.confirm.yes "Yes, delete"

// title when requesting input value through virtual keyboard
input.name "Name"
input.number "Value"
input.hexcolor "Hex color"

// ----------------------------------------------
// postprocess
postprocess.mainEnable "Post Process"
postprocess.quality "Quality"
postprocess.quality.help "Activate these options to improve the quality to the detriment of performance.

It will improve:
- Reflection
- Subsurface
- Ambient Occlusion
- Depth Of Field"
postprocess.maxSamples "Max samples"
postprocess.fullResolution "Full Resolution"
postprocess.accumulateCount "Max frame sampling"
postprocess.accumulateCount.help "Maximum number of frames to accumulates before the rendering stops.

When the rendering stops, Nomad won't do anything, thus saving battery life.

Many rendering features benefits from frame accumulation, notably:
- Soft Shadows
- Reflection
- Subsurface
- Ambient Occlusion
- Depth Of Field

High frame count are only necessary for Soft Shadows.
Depending on features enabled, Nomad can stops the rendering sooner."
postprocess.renderRatio "Render resolution"
postprocess.renderRatioWarning "Overriden by the Pixel Art effect."
postprocess.renderRatio.help "This option impacts the performance a lot.
It's advised to keep a value lower than x1.25.

This option is not saved in the settings."
// fxaa
postprocess.fxaaEnable "Anti-aliasing (FXAA)"
// taa
postprocess.taaEnable "Anti-aliasing (TAA)"
postprocess.taaEnable.help "Reduces flickering when you are moving the camera."
// ssr
postprocess.ssrEnable "Reflection (SSR)"
postprocess.ssrPBRWarning "SSR requires PBR shading mode."
// ssao
postprocess.ssaoEnable "Ambient Occlusion"
postprocess.ssaoRadius "Size"
postprocess.ssaoFactor "Strength"
postprocess.ssaoBias "Curvature bias"
postprocess.ssaoBias.help "How sensitive the effect is depending on the surface curvature."
// dof
postprocess.dofEnable "Depth Of Field"
postprocess.dofBlurFar "Far blur"
postprocess.dofBlurNear "Near blur"
postprocess.dofFocusTip "Tap an object to change the focus point."
// bloom
postprocess.bloomEnable "Bloom"
postprocess.bloomIntensity "Intensity"
postprocess.bloomRadius "Radius"
postprocess.bloomRadius.help "How widespread the bloom is."
postprocess.bloomThreshold "Threshold"
postprocess.bloomThreshold.help "Luminosity threshold to decide if a pixel will emit bloom or not.
If the value is at 0, everything will receive bloom."
// tone mapping
postprocess.toneEnable "Tone Mapping"
postprocess.toneExposure "Exposure"
postprocess.toneContrast "Contrast"
postprocess.toneSaturation "Saturation"
postprocess.toneMappingNone "None"
// curve
postprocess.curveEnable "Color Grading"
postprocess.curve.luminance "Main"
postprocess.curve.red "Red"
postprocess.curve.green "Green"
postprocess.curve.blue "Blue"
postprocess.curveReset "Reset"
postprocess.curveResetAll "Reset All"
// chromatic
postprocess.chromaticEnable "Chromatic Aberration"
postprocess.chromaticFactor "Strength"
// vignette
postprocess.vignetteEnable "Vignette"
postprocess.vignetteSize "Size"
postprocess.vignetteHardness "Hardness"
// sharpness
postprocess.sharpnessEnable "Sharpness"
postprocess.sharpnessFactor "Strength"
// grain
postprocess.grainEnable "Grain"
postprocess.grainFactor "Strength"
// curvature
postprocess.curvatureEnable "Curvature"
postprocess.curvatureCavity "Cavity"
postprocess.curvatureBump "Bump"
// pixel art
postprocess.pixelArtEnable "Pixel Art"
// scanline
postprocess.scanlineEnable "Scanline"
postprocess.scanlineFactor "Factor"
postprocess.scanlineSpacing "Spacing"

// ----------------------------------------------
// primitive (scene menu)
primitive "Primitive"
primitive.box "Box"
primitive.sphereCube "Sphere"
primitive.sphereUV "UV Sphere"
primitive.icosahedron "Icosahedron"
primitive.cylinder "Cylinder"
primitive.cone "Cone"
primitive.torus "Torus"
primitive.lathe "Lathe"
primitive.tube "Tube"
primitive.plane "Plane"
primitive.triplanar "Triplanar"
primitive.faceXYZ "Face XYZ"
primitive.faceXYZ.help "UV-unwrapped base mesh provided by https://texturing.xyz/"
primitive.needValidate "Primitives should be validated in order to be sculpted."

// for 3d editing in viewport
primitive.useFloatPanel "Panel inside viewport"
primitive.useFloatPanel.help "Move some of the primitives options directly in the viewport."
primitive.edit "Edit"
primitive.edit.help "Allow 3d editing in the viewport.

You can disable this feature if you want to interact with the Gizmo or the Transform tool modifying the primitive."

primitive.mainConfig "Parameter"
primitive.topology "Topology"
primitive.geometry "Geometry"

// common config
primitive.validate "Validate"
primitive.maxFaces "Max faces"
primitive.maxFaces.help "The maximum number of faces a primitive can have.

This limit is only active while the primitive is not validated, afterwards the safeguard is gone."
primitive.linear "Linear subdivision"
primitive.subdivision "Post subdivision"

// common config
primitive.radius "Radius"
primitive.size "Size"
primitive.sizeX "Size X"
primitive.sizeY "Size Y"
primitive.sizeZ "Size Z"
primitive.division "Division"
primitive.divisionX "Division X"
primitive.divisionY "Division Y"
primitive.divisionZ "Division Z"
primitive.angleX "Angle X"
primitive.angleY "Angle Y"
primitive.angleZ "Angle Z"
primitive.constantDensity "Constant density"
primitive.projectOnSphere "Project on sphere"
primitive.projectOnSphere.help "Snaps the points on a perfect sphere."

// triplanar
primitive.triplanar.title "Triplanar - Setting"
primitive.triplanar.title.help "Triplanar is using the mask information from 3 planes to fill a voxel grid that is then polygonized.

If you interact with the division or size sliders, the painting information will reset (smoothness is ok).

You should probably disable symmetry as it might not function as you would expect.

You can use the 'Topologically connected' option in the mask panel to paint a plane impacting the other planes."
primitive.triplanarSameSize "Same size (cube)"
primitive.triplanarPolish "Smoothness"
primitive.triplanarResetMask "Reset mask"
primitive.triplanarReproject "Resize mask"
primitive.triplanarReproject.title "Reproject the plane mask when updating the triplanar settings.

If you uncheck this option, it will revert to the default spherical masks."
primitive.isolate.all "All"
primitive.isolate.back "Back"
primitive.isolate.right "Right"
primitive.isolate.bottom "Bottom"
// plane
primitive.planeSameSize "Same size (square)"
primitive.planeDisk "Disk"
// box
primitive.boxRegular "Same size (cube)"
// tube
primitive.tubeSnapOffset "Snap offset"
primitive.tubeSnapOffset.help "A value of 1.0 is equal to the tube's radius."
primitive.tubeThicknessStart "Radius start"
primitive.tubeThicknessEnd "Radius end"
primitive.tubeTwist "Twist"
primitive.tubeTwistRotate "Rotation"
primitive.tubeTwistRadius "Magnitude"
primitive.tubeTwistOffset "Offset"
primitive.tubeSnap "Snap"
// lathe
// torus
primitive.torusRadiusOuter "Radius outer"
primitive.torusRadiusInner "Radius inner"
primitive.torusAngle "Angle"
primitive.torusAngleOffset "Angle offset"
// cylinder
primitive.cylinderHeight "Height"
// cone
primitive.coneRadius "Radius"
primitive.coneHeight "Height"
// hole sub menu (cylinder, tube, etc)
primitive.hole "Hole"
primitive.hasHole "Has hole"
// both used for hole radius and main radius
primitive.radiusSync "Same radius"
primitive.radiusStart "Radius start"
primitive.radiusEnd "Radius end"
primitive.editRadius "Radius"
// spline (for lathe and tube)
primitive.spline "Spline"

// common resources stuffs
resource.delete "Delete"
resource.import "Import..."

// ----------------------------------------------
// scene
scene.title "Scene"
scene.title.help "When using the selection checkbox, hold and drag your finger to select other objects easily."
scene.mergeSimple "Simple merge"
scene.mergeVoxel "Voxel merge"
scene.voxelResolution "Resolution"
scene.subtractionTip "Subtraction: Hide mesh (eye icon)"
scene.intersectionTip "Intersection: All meshes hidden"

// ----------------------------------------------
// settings
settings.displayTitle "Display settings"
// wireframe
settings.wireframeTitle "Wireframe"
settings.wireframeDisplay "Wireframe"
settings.debugUV "Debug UV"
settings.debugUV.help "This option is only relevant if the model has UVs.

It will display the UV wireframe in the background.

It will also display a colored checkerboard texture on the model."
// backface
settings.backfaceTitle "Two sided"
settings.backfaceVisible "Two sided"
settings.backfaceVisible.help "Faces will be visible from both sides."
settings.backfaceColor "Backface color"
settings.backfaceColored "Colored backface"
// outline
settings.outlineTitle "Outline"
settings.outlineEnable "Outline"
settings.outlineThickness "Thickness"
// snap cube
settings.snapCubeTitle "Snap cube"
settings.snapCubeDisplay "Snap cube"
settings.snapCubeBottom "Bottom"
settings.snapCubeLeft "Left"
// stats
settings.statsTitle "Stats"
settings.statsDisplay "Stats"
settings.statsRight "Right"
settings.statsAll "Show full scene"
// grid
settings.gridTitle "Grid"
settings.gridDisplay "Grid"
// cursor
settings.cursorWhileSculpting "Show circle while sculpting"
settings.cursorShowDot "Show small dot"
settings.cursorShowDot.help "The dot can appear as the camera pivot point or when you are sculpting."
settings.cursorShowRope "Show rope stabilizer"
// highlight
settings.highlightSelectionTitle "Highlight selection"
settings.highlightSelection "Highlight selected meshes"
settings.highlightDuration "Duration"
// other
settings.darkenUnselected "Darken unselected meshes"
settings.smoothShading "Smooth shading"
settings.partialDraw "Partial drawing"
settings.partialDraw.help "Feature not polished!

Use it if you are sculpting a relatively small part of a high poly mesh.

It should make the sculpting smoother, but you should not enable wireframe!

Also it might add visual artefacts during the brush strokes"
settings.partialDrawWarning "Do not forget to turn off this option if the visual artefacts are too bothersome!"
settings.showPainting "Show painting"
settings.lightIcon "Light icons"
settings.lightIcon.help "Display light icons on the canvas so that you can select and edit them directly."
settings.holeTitle "Hole-filling"
settings.holeNonManifold "Fill non-manifold"
settings.holeNonManifold.help "Try to fill non manifold hole.
This option is not saved in the settings."
settings.loadGuiSettings "Keep gui settings"
settings.loadGuiSettings.help "When opening or importing a project file, all the gui-related settings embedded in the project will be loaded."
settings.loadObjSplitByGroup "Split OBJ by groups"
settings.loadObjSplitByGroup.help "When enabled, Nomad will split the OBJ groups into separate objects."
settings.loadMergeLayers "Merge Layers"
settings.loadSkipTextures "Skip textures"
settings.loadKeepTopology "Keep topology"
settings.loadKeepTopology.help "Use this option if you don't want Nomad to fiddle with the topology of imported mesh.

It will disable vertex/face reordering, removal of vertex/face duplicates and removal of unused vertices."
settings.loadReverseVertices "Reverse threshold"
settings.loadReverseVertices.help "In order to use less memory, Nomad doesn't save the lower resolution of a mesh.

However it will rebuild the lower resolution if the number of vertices is lower than this threshold."
// multires
settings.multiresTitle "Multiresolution"
settings.multiresMaxVertices "Max vertices count"
settings.multiresMaxVertices.help "Nomad doesn't perform memory check before subdivision, high poly count can easily lead to crashes."
settings.multiresLowResVertices "Low resolution threshold"
settings.multiresLowResVertices.help "A lower resolution of the mesh can be displayed when you move the camera.

You can increase this value if you want to display a higher resolution of the mesh."

// ----------------------------------------------
// shading
shading "Shading"
// main render mode
shading.pbr "Lit (PBR)"
shading.pbr.help "In this mode you can add lights (with shadows), along with an HDR environment.

You can also paint metalness and roughness, thus allowing a finer control on the look of your material."
shading.matcap "Matcap"
shading.matcap.help "Stands for MATerial CAPture, a matcap takes care of both the lighting and material information in a single image.

This is a fast rendering mode, mostly suited for raw sculpting."
shading.unlit "Unlit"
shading.unlit.help "Solid color shading mode, without lighting."
// textures
shading.textures "Use textures"
shading.textures.help "At the moment, you cannot create nor edit textures inside Nomad.

But if you import a file with textures in it, it should work.

-- Supported textures --
Opacity: Lit, Matcap, Unlit
Normal: Lit, Matcap
Color: Lit, Unlit
Emissive: Lit
Roughness: Lit
Metalness: Lit"
// lights
shading.lights "Lights"
shading.lights.addLight "Add light"
shading.lights.pbrWarning "Lights requires PBR shading mode."
// environment
shading.environment "Environment"
shading.environmentImport "Import HDR"
shading.environmentExposure "Exposure"
shading.environmentBackgroundBlur "Blur (background)"
shading.environmentRotation "Rotation"
shading.environmentRotation.help "You can rotate the environment by dragging 3 fingers horizontally on the viewport."
shading.environmentAttachedToCamera "Attached to camera"
shading.environmentAttachedToCamera.help "Attach the environment to the camera.

It will force the lighting to be consistent, which can be useful for sculpting purposes."
// matcap
shading.matcap "Matcap"
shading.matcapRotation "Rotation"
shading.matcapRotation.help "You can rotate the matcap by dragging 3 fingers horizontally on the viewport."
shading.matcapGlobal "Use global matcap"
shading.matcapGlobal.help "Uncheck this option to use a different matcap for this particular mesh."

// ----------------------------------------------
// bottom shortcut buttons (ICON FIT)
shortcut.maskVisible "Mask"
shortcut.solo "Solo"
shortcut.voxelRemesh "Voxel"
shortcut.wireframe "Wire"
shortcut.cameraReset "Reset"
shortcut.cameraSnap "Snap"
shortcut.lockSelection "Lock"
shortcut.perspective "Persp"
shortcut.grid "Grid"
shortcut.uv "uv"

// can be longer (customization name in Interface menu)
shortcut.voxelRemesh.long "Voxel remesh"
shortcut.wireframe.long "Wireframe"
shortcut.cameraReset.long "Camera reset"
shortcut.cameraSnap.long "Camera snap"
shortcut.lockSelection.long "Lock selection"
shortcut.lockSelection.long.help "When enabled, you cannot change the selection by tapping on a mesh."
shortcut.perspective.long "Perspective"
shortcut.grid.long "Grid"

// ----------------------------------------------
// stat
stat.ramScene "Scene"
stat.vramScene "Vram Scene"
stat.vramRender "Vram Render"
stat.vramTextures "Vram Textures"
stat.ramHistory "History"
stat.ramOther "Other"
stat.usedMemory "Used Memory"
stat.freeMemory "Free Memory"
stat.ram "RAM"
stat.used "Used: $0 MB"
stat.free "Free: $0 MB"
stat.faces "Faces"
stat.triangles "Triangles"
stat.vertices "Vertices"
stat.quads "Quads"
stat.sceneFaces "Scene faces"
stat.sceneVertices "Scene vertices"

// ----------------------------------------------
// stroke
stroke "Stroke"
strokeTitle "Stroke ($0)"
stroke.useWorldRadius "World-space radius"
stroke.useWorldRadius.help "This is shared among every tools."
stroke.useShareRadius "Share radius"
stroke.useShareRadius.help "Share the radius value among every tools."
stroke.minSpacingAdjustIntensity "Adjust spacing intensity"
stroke.minSpacingAdjustIntensity.help "Adjust the brush intensity to ensure consistent deformation depending on the stroke spacing."
stroke.minSpacing "Stroke spacing"
stroke.minSpacing.help "Spacing between each stroke, relative to the tool radius.

Lower value will allow smoother stroke but performance will degrade."
stroke.lazySmooth "Stroke smoothing"
stroke.lazySmooth.help "Average multiple pointer position to get a smoother stroke.

With high values, the stroke will lag behind the pointer but will eventually catch up."
stroke.lazyRadius "Lazy rope stabilizer"
stroke.lazyRadius.help "Strokes will lag behind the pointer position according to a certain distance.

This can be used to draw smooth lines."
stroke.globalSettings "This is a global setting"
stroke.snapRadius "Snap radius"
stroke.snapRadius.help "Snap the stroke if the pointer lies close to the last recorded stroke.

This can be useful when drawing long continuous lines, while doing frequent pauses."
stroke.sculptOffset "Stroke offset"
stroke.sculptOffset.help "Apply a constant offset on the stroke.

This option is there to help for small screen when using fingers, so that your finger doesn't cover the stroke."
stroke.accumulate "Accumulate stroke"
stroke.accumulate.help "If this option is enabled, there is no limit to how much matter you can add/remove per stroke."
stroke.useDynamicTopology "Allow dynamic topology"
stroke.connectedTopology "Connected topology"
stroke.connectedTopology.help "This option will only sculpt the vertices that are linked to the picked surface.

This is typically used for the Move tool, for example if you want to exclusively move a part that self-intersect with another part."
stroke.onlyFrontFace "Front-facing vertex only"
stroke.onlyFrontFace.help "This option will ignore back facing vertices.

It can be useful if you want to paint part of a thin geometry without impacting the other side.

It also works for sculpting but you might experience some artefacts."
stroke.onlySameSide "Same-side vertex only"
stroke.onlySameSide.help "Ignore vertices that points in the opposite direction of the deformation."
stroke.curveFalloff "Falloff"
stroke.onlyLasso "Settings only active for the lasso tool."
// alpha
stroke.alpha "Alpha"
stroke.alphaInvert "Invert value"
stroke.alphaWrap "Tiling"
stroke.alphaWrap.none "None"
stroke.alphaWrap.repeat "Repeat"
stroke.alphaWrap.mirror "Mirror"
stroke.alphaProject "Method"
stroke.alphaProject.surfaceContinuous "Surface"
stroke.alphaProject.screenFixed "Screen project"
stroke.alphaTiling "Tiling"
stroke.alphaScale "Scaling"
stroke.alphaScale.help "At minimum value, the alpha square is inside the tool circle radius."
stroke.alphaMidValue "Mid value"
stroke.alphaMidValue.help "Middle-point value at which no deformation occurs.

(Mid value = 0)
- Black: no displacement
- White: positive displacement

(Mid value = 0.5)
- Black: negative displacement
- White: positive displacement

(Mid value = 1)
- Black: negative displacement
- White: no displacement"
// stroke type
stroke.strokeType "Stroke type"
stroke.strokeTypeDot "Dot"
stroke.strokeTypeDrag "Drag"
stroke.strokeTypeGrab "Grab"
stroke.strokeTypeGrabRadius "Grab - dynamic radius"
stroke.strokeTypeGrabIntensity "Grab - dynamic intensity"

// ----------------------------------------------
// symmetry
symmetry "Symmetry"
symmetry.enable "Enabled"
symmetry.plane.title "Planes"
symmetry.toolIgnore "The current tool ignores symmetry."
symmetry.radial.title "Radial"
symmetry.radialX "Radial X"
symmetry.radialY "Radial Y"
symmetry.radialZ "Radial Z"
// method
symmetry.method "Method:"
symmetry.method.help "-- Local
The symmetry plane will move along the mesh when you use one of the transform tools (rotate, translate or gizmo).


-- World
The symmetry plane is fixed and will not move."
symmetry.methodWorld "World"
symmetry.methodLocal "Local"
// flip
symmetry.flip "Flip object"
// mirror
symmetry.mirror "Mirroring"
symmetry.mirror.help "Try to re-apply the symmetry without impacting the topology.

Radial symmetry will be ignored.

If the topology can't be kept because it is not considered symmetrical, you'll get the option to enforce the mirroring."
symmetry.mirrorLeftToRight "Left to Right"
symmetry.mirrorRightToLeft "Right to Left"
symmetry.mirrorFail "Failed to apply symmetry.

Do you want to enforce symmetry by mirroring the mesh?"
symmetry.mirrorUseMasking "Protect masked area"
symmetry.mirrorUseMasking.help "Keep masked area intact.

This option will be ignored with non-symmetric topology (or disconnected surface, like a pair of eyes)."
// reset
symmetry.reset "Reset"
symmetry.resetCenterMesh "Mesh center"
symmetry.resetCenterWorld "World center"
symmetry.resetDirection "Orientation"
// advanced
symmetry.showLine "Show line"
symmetry.showPlane "Show plane"
symmetry.editWarning "Symmetry edit is experimental."
symmetry.edit "Gizmo edit"
symmetry.edit.help "You can freely set the symmetry plane.

This feature is a bit experimental and you should probably never use it."

// ----------------------------------------------
// tools icons on the left bar (ICON FIT)
tool.dynTopo "DynTopo"
tool.symmetry "Sym"
tool.mirror "Mirror"
tool.clay "Clay"
tool.clay.sub "Sub"
tool.brush "Brush"
tool.move "Move"
tool.move.normal "Normal"
tool.drag "Drag"
tool.smooth "Smooth"
tool.smooth.relax "Relax"
tool.mask "Mask"
tool.mask.unmask "Unmask"
tool.maskSelector "SelMask"
tool.smudge "Smudge"
tool.flatten "Flatten"
tool.flatten.fill "Fill"
tool.layer "Layer"
tool.crease "Crease"
tool.trim "Trim"
tool.split "Split"
tool.project "Project"
tool.inflate "Inflate"
tool.pinch "Pinch"
tool.nudge "Nudge"
tool.stamp "Stamp"
tool.clearLayer "DelLayer"
tool.lassoSelect "Select"
tool.gizmo "Gizmo"
tool.gizmo.auto "Auto"
tool.gizmo.editPivot "Pivot"
tool.gizmo.rotateSnap "Snap"
tool.gizmo.local "Local"
tool.transform "Transform"
tool.transform.move "Move"
tool.transform.rotate "Rotate"
tool.transform.scale "Scale"
tool.transform.snap "Snap"
tool.measure "Measure"
tool.view "View"
tool.lathe "Lathe"
tool.tube "Tube"
tool.insert "Insert"
tool.shape.flip "Flip"
tool.shape.view "View"
tool.shape.lasso "Lasso"
tool.shape.curve "Curve"
tool.shape.polygon "Polygon"
tool.shape.path "Path"
tool.shape.rectangle "Rect"
tool.shape.ellipse "Ellipse"
tool.shape.line "Line"
tool.shape.closed "Closed"

// popup when editing sliders
tool.sliderRadius "Radius $0"
tool.sliderIntensity "Intensity $0 %"

// ----------------------------------------------
// title
tool.settingsTitle "Settings ($0)"

// ----------------------------------------------
// tool menu
tool.noSettings "This tool doesn't have any specific settings."

// ----------------------------------------------
// clay
tool.clay.flattenOffset "Flatten offset"

// ----------------------------------------------
// crease
tool.crease.pinchFactor "Pinch force"

// ----------------------------------------------
// layer
tool.layer.removeInfluence "Use current layer offset"
tool.layer.removeInfluence.help "This option is only active when there is a current layer selected.

It will use the current layer offset to limit the displacement over strokes."
tool.layer.noLayerSelected "This option is only available if a current layer is selected"

// ----------------------------------------------
// flatten
tool.flatten.warning "These options are experimental and could be removed in the future!"
tool.flatten.planeLockOrigin "Lock plane origin"
tool.flatten.planeLockNormal "Lock plane direction"
tool.flatten.planeAverageOrigin "Average plane origin"
tool.flatten.planeAverageNormal "Average plane direction"
tool.flatten.planeOffset "Plane offset"

// ----------------------------------------------
// smooth
tool.smooth.stickyBorder "Sticky vertex on border"

// ----------------------------------------------
// paint
tool.paint "Paint"
tool.paint.erase "Erase"
tool.paint.depthFilter "Depth filtering"
tool.paint.layerFilter "Layer filtering"
tool.paint.layerFilter.help "Use this option if you only want to repaint the already painted area of a layer."

// ----------------------------------------------
// masking
tool.mask.clear "Clear"
tool.mask.invert "Invert"
tool.mask.flipConnected "Flip connected"
tool.mask.blur "Blur"
tool.mask.sharpen "Sharpen"
tool.mask.thickness "Shell thickness"
tool.mask.polish "Border smoothness"
tool.mask.engraveEmboss "Engrave / Emboss"
tool.mask.extract "Extract"
tool.mask.split "Split"
tool.mask.closeMask "Closing action (masked):"
tool.mask.closeUnmask "Closing action (unmasked):"
tool.mask.closeAction "Closing action:"
tool.mask.closeActionNone "None"
tool.mask.closeActionFill "Fill"
tool.mask.closeActionShell "Shell"
tool.mask.closeActionLayer "Layer"
tool.mask.closeAction.help "-- None
Simply extract the part and let the extracted part opened.

-- Fill
Hole is filled and smoothed.
Do not use this option for flat surface.

-- Shell
Close the extracted shape by using the thickness value.

-- Layer
Extract the layer difference (layer sub-menu only)."

// ----------------------------------------------
// matrix (transform / gizmo)
tool.matrix "Matrix"
tool.matrix.clone "Clone"
tool.matrix.action "Action"
tool.matrix.action.help "-- Move origin
Move the mesh to the world origin.

-- Reset
Reset the mesh transform to identity.

-- Bake
Apply the matrix to the vertex and reset the matrix. Visually, nothing should change."
tool.matrix.transformOperation "Transform operation"
tool.matrix.translation "Translation"
tool.matrix.rotation "Rotation"
tool.matrix.scale "Scale"
tool.matrix.uniformScale "Uniform scale"
tool.matrix.uniformScale.help "Nomad cannot support non-uniform scale as object transform, so it will be applied as a vertex transformation."
tool.matrix.moveToOrigin "Move origin"
tool.matrix.resetTransform "Reset"
tool.matrix.bakeTransform "Bake"
tool.matrix.applyMethod "Method:"
tool.matrix.applyMethodAuto "Auto"
tool.matrix.applyMethodVertex "Vertex"
tool.matrix.applyMethodObject "Object"
tool.matrix.applyMethod.help "-- Auto
Let Nomad choose between Vertex or Object mode.
Typically, object is preferred unless symmetry is enabled or if there is masking on the mesh.

-- Vertex
Vertices are transformed individually.
Symmetry and mask are taken into account.
For primitives that are not validated, Object mode is forced.

-- Object
The object is transformed as a whole.
Symmetry and mask are ignored.
If you use non-uniform scaling, Vertex mode will be forced."

// ----------------------------------------------
// transform
tool.transform.multiTouch "Multi-touch"
tool.transform.multiTouch.help "If this option is disabled, you can only use one mode (translate, rotate, scale) at a time."
tool.transform.transformRestrictRotationY "Y axis rotation"
tool.transform.transformRestrictRotationY.help "Auto value means the option is active only if Snap is on and the mesh has a custom gizmo pivot."

// ----------------------------------------------
// gizmo
tool.gizmo.size "Widget size"
tool.gizmo.linearRollThreshold "Tangent roll threshold"
tool.gizmo.linearRollThreshold.help "Angle threshold to choose between linear or circular roll method.

Value above this threshold will use the circular roll.

If you prefer the linear roll (direction of the tangent), simply set this value to 90°."
tool.gizmo.autoHide "Hide on interaction"
tool.gizmo.tap "Move custom pivot on single-tap"
tool.gizmo.tap.help "This option is only effective in custom pivot mode (Auto disabled).

-- None
Nothing happen when tapping on the mesh.

-- First hit
Move the gizmo on the first intersection.

-- Middle stab
Move the gizmo on the average of the first two intersections."
tool.gizmo.tapNone "None"
tool.gizmo.tapFirstHit "First hit"
tool.gizmo.tapMiddleStab "Middle stab"

// ----------------------------------------------
// lathe
tool.lathe.axis "Axis"
tool.lathe.axis.fixed "Fixed"
tool.lathe.axis.dynamic "Dynamic"

// ----------------------------------------------
// tube
tool.tube.snap "Snapping"
tool.tube.snap.all "Every point"
tool.tube.snap.startEnd "Start & End"

// ----------------------------------------------
// trim
tool.hole "Hole filling"
tool.hole.fillHoles "Fill holes"
tool.hole.bridges "Screen-space boolean"
tool.hole.bridges.help "If this option is enabled, you can punch holes in the volume.
The cut slope will also follow more closely the cutting shape."
tool.hole.threshold "Threshold epsilon"
tool.hole.threshold.help "Tweaking this value might help with the hole filling algorithm."
tool.hole.smoothing "Hole smoothing"

// ----------------------------------------------
// smudge
tool.smudge.quality "Quality"
tool.smudge.quality.help "It changes the resolution of the projected pixels, lower values means faster strokes."

// ----------------------------------------------
// trim / split / project / selMask
tool.shape "Shape"
tool.shape.rectangleSquare "Square"
tool.shape.rectangleCentered "Centered"
tool.shape.ellipseCircle "Circle"
tool.shape.ellipseCentered "Centered"
tool.shape.lineRotateStep "Rotate step"

// ----------------------------------------------
// measure
tool.measure.goldenRatio "Show golden ratio"

// ----------------------------------------------
// topology
topology "Topology"
// multires
topology.multires.title "Multiresolution"
topology.multires.title.help "Keep multiple resolution of a mesh.

If you make changes in a lower resolution, details from the higher resolutions will be reprojected when you switch back.

Layers are available on every resolution."
topology.multiresReverse "Reverse"
topology.multiresReverse.confirm "Could not create base subdivision.

The current topology is probably not a result from a subdivision."
topology.multiresSubdivide "Subdivide"
topology.multiresSubdivideConfirm "The mesh will have $0M vertices, are you sure?"
topology.multiresDeleteLower "Delete lower"
topology.multiresDeleteHigher "Delete higher"
topology.multiresKeepTriangles "Keep triangles"
topology.multiresLinear "Linear subdivision"
topology.multiresLinear.help "Simply subdivide the mesh without applying any smoothing"
// voxel
topology.voxel.title "Voxel remeshing"
topology.voxel.title.help "Remeshing by sampling the mesh on a grid.

If the object is not closed (watertight), an hole-filling algorithm will be applied first.

Layers are reprojected after remeshing but the quality will degrade."
topology.voxelResolution "Resolution"
topology.voxelRemesh "Remesh"
topology.voxelSharp "Keep sharp edges"
topology.voxelSharp.help "This option is mostly useful for simple primitive boolean operation.

It will introduce distortion in some area due to the points being snapped on the edges."
topology.voxelSubLevel "Build multiresolution"
topology.voxelSubLevel.help "You can rebuild a multiresolution hierarchy from the voxel remesher output.

It will also run faster and use less memory, especially if the voxel detail value is high.
However if the voxel detail value is low and you are asking for lot of multires level, you'll lose details."
// dynamic topology
topology.surfaceUniform "Remesh"
topology.surfaceDetail "Detail"
topology.surfaceDetail.help "Unlike voxel remeshing, surface remeshing doesn't require the mesh to be closed.

It can also support masking so that you can protect some part of the mesh from topology changes.

Layers are updated correctly."
topology.surfaceMethod "Method"
toplogy.surfaceMethodUniformisation "Uniformisation"
toplogy.surfaceMethodSubdivision "Subdivision"
toplogy.surfaceMethodDecimation "Decimation"
topology.surfaceMethod.help "Behavior of dynamic topology:
- Uniformisation: add and remove detail
- Subdivision: add detail
- Decimation: remove detail"
topology.surfaceUseMasking "Protect masked area"
topology.surfaceUseMasking.help "The masked areas will protect the topology from beging changed."
topology.surfaceExtrapolate "Vertex extrapolation"
// dynamic
topology.dynamic "Dynamic topology"
topology.dynamicActivate "Enabled"
topology.dynamicActivate.help "With dynamic topology, sculpting tools can subdivide or simplify the mesh locally in real time.

This feature can have a noticeable impact on performance.

Layers are updated correctly."
topology.dynamicDetailLevel "Detail"
topology.dynamicDetailEdge "Detail"
topology.dynamicDetailMethod "Detail based on..."
topology.dynamicDetailMethodZoom "Zoom"
topology.dynamicDetailMethodRadius "Radius"
topology.dynamicDetailMethodConstant "Constant"
topology.dynamicDetailMethod.help "-- Zoom
The level of detail is based on how far you are from the surface.

-- Radius
The tool radius defines the amount of detail.

-- Constant
The detail is fixed, the detail value is shared with the voxel slider as well."
topology.dynamicQuality "Prefer..."
topology.dynamicQuality.help "If you choose Quality, the 2 main differences are:
- refinement is applied before the sculpting operator, you will get less interpolating artefact when painting or sculpting very small details
- refinement is not applied incrementally, if you sculpt very small details or do quick strokes, the topology will always be correctly refined

For better performance, and if you plan on using this option, you might consider enabling the \"partial drawing\" option in the Settings panel."
topology.dynamicQualitySpeed "Speed"
topology.dynamicQualityQuality "Quality"
topology.dynamicUsePressure "Use pressure on radius"
topology.dynamicUsePressure.help "Use this option if you want the pen pressure impact on tool radius to impact the level of detail."
// decimate
topology.decimate.title "Decimation"
topology.decimate.title.help "Reduce the number of polygons by trying to keep as many details as possible.

This feature can be useful if you want to export for 3d printing.
However you should probably not use it if you want to continue sculpting on it, as it can produce uneven triangles.

Note that the masked area won't be decimated."
topology.decimate "Decimate"
topology.decimateTargetFaces "Target triangles"
topology.decimatePaintWeight "Preserve painting"
topology.decimatePaintWeight.help "Higher value will try to preserve painting.

Set this value to 0 if you don't care about the painting."
topology.decimateUniform "Uniform faces"
topology.decimateUniform.help "Higher value will output triangles with similar size."
// topology.decimatePreserveBorders "Preserve borders"
// topology.decimatePreserveBorders.help "Do not decimate the border of the mesh.

// This is only relevant for object that are not watertight."

// BFF is activated through Debug menu
topology.uv.title "UV Auto-Unwrap"
topology.uvAtlas "Unwrap Atlas"
topology.uvAtlas.warning "Can be very slow, target <100k vertices!"
topology.uvBFF "Unwrap Bff"
topology.uvBFF.warning "Can have overlaps if mesh has handles!"
topology.uvBFFCones "Cone count"
topology.uvBFFCones.help "Higher value will reduce distortion for complex objects.

Higher value will means longer compute time."
topology.uvDelete "Delete UVs"

// baking
topology.bake "Bake Vertex painting"
topology.bake.help "Transfer the vertex painting to a texture.

Vertex color are resetted in the process."
topology.bakeResolution "Resolution"

// ----------------------------------------------
// privacy policy
privacyPolicy.title "Privacy Policy"
privacyPolicy.reject "Reject"
privacyPolicy "Hexanomad does not collect any data from Nomad Sculpt."

// ----------------------------------------------
// version trial
version.buyWeb "Web version is only a demo"
version.buyFull "Upgrade to full version"
version.restorePurchase "Restore purchase"
// version.promoHuawei ""

version.trialHistory "Trial version: 4 undo/redo possible"
version.trialLayer "Trial version: 1 layer per mesh"
version.trialOneProject "Trial version: 1 active project only"
version.trialNoImport "Trial version: No import"
version.trialNoExport "Trial version: No export"

version.fullFeatures "- One time purchase
- Unlimited undo/redo
- Unlimited layers
- Save & Load
- Export & import"