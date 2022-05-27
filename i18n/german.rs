// Comments and entries are sync from the english version, it's not
// possible to have language specific comments at the moment.
// You can use this entry to make a specific comment.
language_note "german translation by djblueprint / www.3d-board.de"

// comments with "ICON FIT" should be short, ideally < 10 characters

// When in doubt, leave an empty string, it will fallback to english
// Some terms should probably be left untranslated
// For sure: Voxel, Matcap, DynTopo, PBR, Dyntopo
// Not sure: Roughness/Metalness? Mesh? Sub? tool names? etc

// ----------------------------------------------
// general stuffs

// Popup question, confirm? [yes/cancel] [ok/cancel] [delete/cancel]
confirm "Bestätigen?"
yes "Ja"
ok "OK"
delete "Löschen"
cancel "Abbrechen"

// feature: Auto / Off / On
on "An"
off "Aus"
auto "Auto"

// coordinate
X "X"
Y "Y"
Z "Z"

// general
advancedSettings "Erweitert"
notSaved "Diese Optionen werden nicht in den Einstellungen gespeichert"

// generic warning when there is no mesh selected
noSelectedMesh "Kein Mesh ausgewählt"

// generic warning when only one mesh needs to be selected
multipleObjectWarning "Mehrere Meshes sind ausgewählt, bitte nur ein Mesh auswählen."

// ----------------------------------------------

// when you launch the app and there is missing Nomad/data files
loading.reprocess "Fehlende Vorschaubilder, erstelle neu... ($0/$1)

$2"

// main pbr channel
baseColor "Color"
roughness "Roughness"
metalness "Metalness"

// ----------------------------------------------
// about
about.minify "Menüs ausblenden"
about.minify.help "Sie können auch mit 4 Fingern auf den Bildschirm tippen, wenn Ihr Gerät dies unterstützt."
about.turntable "Turntable"
about.turntableSpeed "Geschwindigkeit Turntable"
about.credits "Credits"
about.creditsOpenSource "Open-Source"
about.creditsArts "MatCaps & HDRIs"
about.languages "Sprachen"
about.languages.help "Die Übersetzung ist verfügbar unter github.com/stephomi/nomad-translation"
about.openUrl "Öffnen?"
// nomad
about.website "Webseite"
about.forum "Forum"
about.manual "Handbuch"
about.mail "Support"
// social
about.twitter "Twitter"
about.instagram "Instagram"
about.facebook "Facebook"
about.discord "Discord"

// ----------------------------------------------
// alert
alert.hole.nothing "Das Objekt hat keine Löcher!"
alert.shape.notVisible "Das aktuelle Mesh ist unsichtbar!"
alert.trim.nothing "Nichts zu beschneiden."
alert.trim.full  "Beschneiden abgebrochen: Das Mesh würde vollständig beschnitten werden."
alert.mask.noExtract "Nichts zum Extrahieren!"
alert.mask.noSplit "Nichts zum Teilen ausgewählt!"
alert.view.disabled "Funktionen im Ansichtsmodus deaktiviert:"
alert.view.disabled.widgetPrimitive "Primitive widgets"
alert.separate.fail "Kann nicht getrennt werden: Das Objekt hat nur einen Teil!"
alert.voxelRemesh.success "Remeshing ausgeführt!"
alert.voxelRemesh.empty "Remeshing abgebrochen: Das Ergebnis hätte keine Faces mehr."
alert.voxelRemesh.invalidInput "Ungültige Eingabe!"
alert.matrix.clone "Das Objekt wird dupliziert"
alert.gizmo.usePivot "Benutzerdefinierten Drehpunkt (Pivot) verwenden."
alert.gizmo.useAuto "Automatischen Drehpunkt (Pivot) verwenden."
alert.gizmo.editPivot "Aktueller Modus: Drehpunkt (Pivot) bearbeiten"
alert.gizmo.editObject "Aktueller Modus: Objekt bearbeiten."
alert.dynamic.enable "Dynamic Topology ist aktiviert!"
alert.dynamic.disable "Dynamic Topology ist deaktiviert!"
alert.colorPicker "Ziehen Sie Ihren Finger auf das Mesh, um eine Farbe auszuwählen."
alert.backgroundTransform "Einfaches Antippen, um den Transformationsmodus zu verlassen."
alert.camera.resetView "Reset Ansicht"
alert.camera.snapView "Snap-Ansicht"
alert.mask.show "Maske anzeigen"
alert.mask.hide "Maske ausblenden"
alert.selection.lock "Auswahl sperren"
alert.selection.unlock "Auswahl entsperren"
alert.selection.isolate "Auswahl isolieren"
alert.selection.showAll "Alles anzeigen"
alert.quickSave "Speichere..."
alert.forceShowPainting.fill "Painting anzeigen aktiviert, [Paint all] wurde verwendet."
alert.forceShowPainting.tool "Painting anzeigen aktiviert, Objekt painted."
alert.multiresLost "Multiresolution geht verloren!"
alert.rangeWarning "Der Detailgrad ist hoch und kann viel Speicherplatz erfordern!"
// autosave popup
alert.autoSave.auto "Automatisches Speichern in... $0s"
// bottom warning
alert.warning.needLayer "Das aktuelle Werkzeug erfordert eine aktive Ebene."
alert.warning.paintingHidden "Painting ausgeblendet: Im Einstellungsfenster wieder einblenden."
alert.warning.noPartialWireframe "Das partielle Zeichnen ist deaktiviert, wenn das Drahtgitter (Wireframe) eingeblendet ist."
// bottom tip
alert.tip.shapeOrthographic "Erwägen Sie die Verwendung einer orthografischen Kamera, wenn Sie perspektivische Verzerrungen bei der Verwendung von Screen Project vermeiden möchten."
// undo
alert.state.trial "Rückgängig abgebrochen: Sie benutzen die Testversion!"

// ----------------------------------------------
// background
background "Hintergrund"
background.color "Farbe"
background.environment "Umgebung"
background.blur "Unschärfe"
background.exposure "Belichtung"

background.imageEnable "Referenzbild"
background.imageOverlay "Overlay"
background.imageAlpha "Alpha"
background.imageReset "Reset Einstellungen"
background.imageTransform "Umwandeln"
// transform
background.imageX "X-Position"
background.imageY "Y-Position"
background.imageRotation "Rotation"
background.imageScale "Skalierung"

// ----------------------------------------------
// camera
camera "Kamera"
// saved views
camera.updateView "Ansicht updaten?"
camera.addView "Ansicht hinzufügen"
camera.focusOn "Fokus auf"
// projection
camera.projection "Projektion"
camera.orthographic "Orthographisch"
camera.perspective "Perspektivisch"
camera.fov "Vertikales FoV"
camera.focal "Brennweite $0mm (35mm Sensor)"
// orbit
camera.orbit "Orbit mode"
camera.orbit.help "Trackball gibt mehr einem mehr Spielraum, man kann die Kamera auch mit 2 Fingern drehen."
camera.trackball "Trackball"
camera.turntable "Turntable"
// speed
camera.speed "Geschwindigkeit"
camera.rotation "Rotation"
camera.panning "Schwenken (Pan)"
camera.zooming "Zoomen"
// misc
camera.resetView "Reset Ansicht"
camera.snapView "Snap Ansicht"
// interaction
camera.pivot "Pivot"
camera.doubleTapMesh "Doppel-Tap Mesh"
camera.doubleTapBackground "Doppel-Tap Hintergrund"
camera.doubleTapPivot "Update bei Doppel-Tap"
camera.doubleTapPivot.help "Aktualisieren Sie den Drehpunkt (Pivot) beim doppelten Antippen."
camera.airPivot ""
camera.airPivot.help ""
camera.autoPivot "Bei Kamerabenutzung"
camera.autoPivot.help "Aktualisieren Sie den Drehpunkt (Pivot), wenn Sie beginnen, mit der Kamera zu hantieren."
camera.doubleTapFocus "Fokus"
camera.doubleTapFocus.help "Wenn Sie doppelt auf das Mesh tippen, schwenkt die Kamera und fokussiert auf den ausgewählten Punkt."
camera.doubleTapFocusSelection "Fokus auf Auswahl"
camera.doubleTapFocusSelection.help "Wenn Sie doppelt auf den Hintergrund tippen, wird der Fokus auf das ausgewählte Mesh anstatt auf die gesamte Szene gelegt."

// toolbox context, only a few tools are display in some cases
// (only visible in expanded toolbox mode)
context.multiselection ""
context.triplanar ""
context.primitive ""

// scene and layer lists
curve.preset "Preset"
curve.custom "Benutzerdefiniert"

// ----------------------------------------------
// debug
debug.uvPrimitive.warning "Deaktivieren Sie diese Option, wenn Sie keine UVs benötigen (zusätzlicher Speicher)."
debug.uvPrimitive "Primitive UVs beibehalten"
debug.uvPrimitive.help "Momentan werden nur Box und Kugel (Sphere) unterstützt.

Andere Typen werden in Zukunft unterstützt werden."
debug.uvNormalize "UVs normalisieren"
debug.uvNormalize.help "Nomad normalisiert die UVs innerhalb Tile [0-1]."
debug.uvBFF "BFF UVs hinzufügen"
debug.uvBFF.help "Hinzufügen einer alternativen Unwrapping-Methode (boundary first flattening - BFF).

Beachten Sie, dass BFF zu Überschneidungen führt, wenn Ihre Mesh-Topologie nicht aus einer Scheibe oder Kugel besteht."
debug.logs "Logs"
debug.heightmap "Heightmap"
debug.graphics "Grafik"
debug.thumbnails "Vorschaubilder erstellen"

// scene and layer lists
expandList "UI: Liste erweitern"
expandList.help "Nur eine UI-Option zur einfacheren Listenverwaltung."

// ----------------------------------------------
// file
file.project.empty "Sie haben noch kein Projekt gespeichert!"
file.project.unsaved "Nicht gespeicherte Änderungen!"
file.project.loseUnsaved "Sie werden nicht gespeicherte Änderungen verlieren!"
file.project.lastManualSave "Vorschau der letzten manuellen Speicherung"
file.project.trialNoOpen "Testversion: Sie können das aktuelle Projekt nicht mehr öffnen!"
file.project.trialOnlyOpen "Testversion: Sie können nur Ihr aktuelles Projekt wieder öffnen!"

// ----------------------------------------------
// project
file.project "Projekt"
file.project.save "Speichern"
file.project.save.confirm "$0 speichern?"
file.project.saveAs "Speichern als"
file.project.saveAs.confirm "$0 überschreiben?"
file.project.open "Öffnen"
file.project.open.confirm "$0 öffnen?"
file.project.add "Hinzufügen"
file.project.add.confirm "$0 der Szene hinzufügen?"
file.project.new "Neu"
file.project.new.confirm "Neue Szene erstellen?"
file.project.delete "Löschen"
file.project.delete.confirm "$0 löschen?"
file.project.delete.confirmActive "$0 löschen?

Dies ist das derzeit aktive Projekt!"
file.project.delete.confirmOk "Sind Sie sicher?"
file.project.rename "Umbenennen"

// autosave
file.project.autoSave "Auto-Speichern"
file.project.autoSave.confirm "Automatisches Speichern deaktivieren?"
file.project.autoSave.help "Speichert Ihr Projekt in regelmäßigen Abständen in einer separaten Datei.
Die automatisch gespeicherte Datei finden Sie in:

$0"
file.project.autoSave.popup "Popup-Zeitüberschreitung"
file.project.autoSave.minutes "Timer Popup"
file.project.autoSave.delete "AutoSave löschen"
file.project.autoSave.delete.confirm "Bestätigen?"

file.importSettings ""

// import
file.import.title "Import"
file.import.title.help "Unterstützte Formate:
- Wavefront (.obj)
- glTF 2.0 (.glb .gltf)
- STL (.stl)"
file.importOpen "Öffnen"
file.importOpen.confirm "Neue Datei importieren?"
file.import.add "Der Szene hinzufügen"
file.import.add.confirm "Neue Datei importieren?"

file.exportSelection "Nur Auswahl exportieren"
file.exportSelection.help "Exportieren Sie nur das aktuell ausgewählte Mesh anstelle der gesamten Szene."
file.convertToQuad "Vierecke wiederherstellen"
file.convertToQuad.help "Wiederherstellen von Vierecken aus Dreiecken durch Paarung von Dreiecken (wenn sie in den Dateien benachbart sind)."

// export
file.export.title "Export"
file.export.title.help "Wählen Sie den glTF-Export falls möglich. Das glTF-Format unterstützt beim Export mehr Funktionen als andere Formate.

Nicht jedes Programm kann allerdings glTF importieren."

// generic export
file.export.texture "Texturen exportieren"
file.export.texture.help "Mit dieser Option werden keine Vertex-Farben in die Texturen eingefügt (kein Baking).

Es werden nur dann Texturen neu exportiert, wenn sie bereits in einer importierten Datei vorhanden waren."
file.export.normal "Normale exportieren"
file.export.normal.help "Aktivieren Sie diese Option, wenn Sie die Datei in einer anderen Software öffnen möchten.

Nomad ignoriert immer die Normale, da es sie neu berechnet."

// gltf
file.export.gltf "glTF 2.0 exportieren"
file.export.gltfLayer "Layer exportieren"
file.export.gltfLayer.help "Exportieren Sie Ebenen als Morphs. Offiziell von glTF unterstützt, so dass es auch in anderen Programmen funktionieren sollte."
file.export.gltfLayerPaint "Zusätzliche Material-Layer exportieren"
file.export.gltfLayerPaint.help "Exportieren Sie Roughness, Metalness, Masken und Layer-Painting. Dies wird von anderen Programmen als Nomad ignoriert werden."
file.export.gltfLayerNomad ""
file.export.gltfLayerNomad.help ""
file.export.gltfColor0 "Vertex-Farben exportieren"
file.export.gltfColor0.help "Exportieren Sie Vertex-Farben. Offiziell von glTF unterstützt, so dass es auch in anderen Programmen funktionieren sollte."
file.export.gltfColor1 ""
file.export.gltfColor1.help ""

// obj
file.export.obj "OBJ exportieren"
file.export.objWarning "Layer und zusätzliches Painting (Roughness, Metalness, Masken) gehen verloren."
file.export.objColorAppend "Vertex-Farben exportieren"
file.export.objColorAppend.help "Farbinformationen nach Vertices einfügen.

Einige 3D-Programme können dies importieren, aber nicht alle."

// stl
file.export.stl "STL exportieren"
file.export.stlWarning "Layer und zusätzliches Painting (Roughness, Metalness, Masken) gehen verloren."
file.export.stlColor "Vertex-Farben exportieren"
file.export.stlColor.help "Einige 3D-Programme können dies importieren, aber nicht alle."
file.export.stlAscii "Standardmäßig ist das Format binär.

Sie können auch in das Textformat (ASCII) exportieren, allerdings wird die Datei dann größer."

file.settings.title "Einstellungen"
file.settings.title.help "Hier werden die meisten Einstellungen der App gespeichert (Kamera, Interface usw.).

Einige Ressourcen werden separat und automatisch gespeichert, dazu gehören:

- HDRIs
- MatCaps
- Alphas
- Texturen
- Hintergründe
- Projekte

Im Moment können die Pinsel-Einstellungen nicht gespeichert werden, aber eine benutzerdefinierte Pinsel-Verwaltung ist geplant."

// settings
file.settings.reset "Reset"
file.settings.reset.confirm "Alle Einstellungen zurücksetzen?

Projekte, Alphas, MatCaps, HDRIs und Hintergründe sind davon nicht betroffen."

// render
file.render "Render"
file.render.showInterface "Interface anzeigen"
file.render.renderRatio "Renderfaktor"
file.render.renderRatio.help "Ein Wert von 1,0 bedeutet, dass Nomad mit der gleichen Auflösung rendert, wie die unten angegebene Bildgröße.

Verwenden Sie diese Option, wenn Sie bei einer bestimmten Auflösung nicht rendern können (Abstürze wegen Speichermangels)."
file.render.help "Renderfaktor"
file.render.size "Endgültige Größe"
file.render.size.custom "Benutzerdefiniert"
file.render.screenResolution "Bildschirm"
file.render.export "PNG exportieren"
file.render.width "Breite"
file.render.height "Höhe"
file.render.warn "Die Export-Auflösung ist hoch ($0x$1)!

Stellen Sie sicher, dass Sie Ihr Projekt vorher speichern, für den Fall, dass Ihr Gerät keinen VRAM mehr hat und abstürzt."
file.render.transparent "Transparenter Hintergrund"
file.render.transparent.help "Diese Option kann nützlich sein, wenn Sie das freigestellte Mesh in eine 2D-Software importieren möchten.

Teilweise Objekttransparenz wird momentan noch nicht unterstützt."

// ----------------------------------------------
// gesture menu
gesture.useGlobal "Globale Vorgabe nutzen"
gesture.useGlobal.help "Standardmäßig haben alle Werkzeuge dieselben Pressure-Settings.

Deaktivieren Sie diese Option, wenn Sie spezielle Pressure-Settings für dieses Werkzeug wünschen."

gesture.pressure "Pressure"
gesture.pressureTitle "Pressure ($0)"
gesture.pressure.noTool "Dieses Werkzeug verwendet keine Pressure-Settings für den Stift."
gesture.pressure.noGrab "Grab ignoriert Pressure-Settings."
gesture.pressure.radius "Radius"
gesture.pressure.intensity "Stärke"
gesture.pressure.useRadius "Aktiv"
gesture.pressure.useIntensity "Aktiv"
gesture.pressure.curveRadius "Radius"
gesture.pressure.curveIntensity "Stärke"

gesture.cameraInteraction "Kamera:"
gesture.sculptInteraction "Sculpt:"
gesture.interaction.fingerAndStylus "Finger und Stylus"
gesture.interaction.finger "Finger"
gesture.interaction.stylus "Stylus"

gesture.fingerLighting "Licht-Rotation (3 Finger)"
gesture.fingerLighting.help "Bewegen Sie 3 Finger horizontal (von links nach rechts oder umgekehrt) auf der dem Arbeitsbereich, um die Umgebung, die Lichter und das MatCap zu rotieren."
gesture.fingerRadius "Tool-Radius bearbeiten (3 Finger)"
gesture.fingerRadius.help "Bewegen Sie 3 Finger horizontal auf der dem Arbeitsbereich (von oben nach unten = verkleinern, von unten nach oben vergößern), um den Werkzeug-Radius zu verändern."

gesture.fingerSmooth "Finger glättet immer"
gesture.unknownPressure "Unerkannten Pressure zulassen"
gesture.unknownPressure.help "Aktivieren Sie diese Option, wenn Pressure (Druck) mit Ihrem Stift nicht funktioniert oder wenn Sie einen Pressure für den Finger benötigen."

// pencil
gesture.pencilAction.none "Nichts"
gesture.pencilAction.smooth "Smooth"
gesture.pencilAction.alt "Add/Sub"
gesture.pencilAction.android "Stift: Taste"
gesture.pencilAction.android.help "Experimentell"
gesture.pencilAction.ios "Stift: Doppel-Tap"
gesture.pencilAction.ios.help "Nur aktiv für Apple Pencil 2. Generation."

// history
gesture.history "Schnelle-Geste"
gesture.history.help "2-Finger Tap für UnDo.

3-Finger Tap für ReDo."

// size rejection
gesture.useSizeRejection "Touch Schwelle"
gesture.useSizeRejectionConfirm "Wenn Sie Probleme bei der Erkennung der Touch-Eingabe haben, stellen Sie sicher, dass diese Option deaktiviert ist!"
gesture.useSizeRejection.help "Die Eingabe auf dem Touchdisplay wird abgelehnt, wenn die Größe der Kontaktfläche beim Touch größer ist, als der vorgegebene Wert.

Funktioniert möglicherweise nicht auf jedem Gerät."
gesture.sizeRejection "Schwelle für maximale Größe"
// help
gesture.interaction.title "Gesten"
gesture.interaction.title.help "Diese Optionen gelten immer global."

// ----------------------------------------------
// history
history "Verlauf"
history.root "Root"
history.undoConfirm "Bestätigen Sie das UnDo (Rückgängig machen) all dieser Änderungen?"
history.undoWarning "Wenn Sie eine nachträgliche Bearbeitung vornehmen, können viele Änderungen verloren gehen."
history.stack "Stapel"
history.limitSize "Verlaufslimit (MB)"
history.limitSize.help "Max. Größe des Verlaufs (in MB).

Der Verlauf wird bei der nächsten aufgezeichneten Änderung aktualisiert."
history.limitStack "Stapel-Limit"
history.limitStack.help "Maximale Anzahl von Änderungen, die im Verlauf bleiben.

Der Verlauf wird bei der nächsten aufgezeichneten Änderung aktualisiert."
history.rangeProtect "Schutz des Bereichs"
history.rangeProtect.help "Wenn Sie weit in der Verlaufsliste zurückgehen, wird ein Bestätigungsdialog angezeigt, bevor viele Änderungen auf einmal rückgängig gemacht werden."
history.restoreCamera "Reset Kamera"
history.restoreCamera.help "Aktivieren Sie diese Option, um die gespeicherte Kamera-Ansicht wiederherzustellen, wenn Sie eine Aktion rückgängig machen/wiederherstellen (UnDo/ReDo)."
// display undo/redo
history.state.undo "UnDo: $0"
history.state.redo "ReDo: $0"
history.state.symmetrySplit "Symmetrie Split"
history.state.voxelRemesh "Voxel Remesh"
history.state.surfaceRemesh "Surface Remesh"
// state multires
history.state.multiresToDynamic "Multires zu Dynamisch"
history.state.multiresLevel "Auflösung ändern"
history.state.multiresSubdivide "Subdivide"
history.state.multiresReverse "Reversion"
history.state.multiresDeleteLower "Low-Res löschen"
history.state.multiresDeleteHigher "High-Res löschen"
// mesh
history.state.meshDynamicToStatic "Dynamisch zu Statisch"
history.state.meshStaticToDynamic "Statisch zu Dynamisch"
history.state.meshSymmetryUpdate "Symmetrie-Update"
history.state.meshMatrixUpdate "Matrix-Update"
history.state.meshVisibility "Sichtbarkeit"
history.state.meshMaterial "Material ändern"
// state scene
history.state.sceneAddRemove "Szene"
history.state.sceneMeshOrder "Mesh Reihenfolge"
// state layer
history.state.layerOrder "Layer $0 Reihenfolge"
history.state.layerMergeRedo "Layer $0 getrennt"
history.state.layerCreate "Layer $0 erstellt"
history.state.layerDelete "Layer $0 gelöscht"
history.state.layerMerge "Layer $0 verschmolzen"
history.state.layerHide "Layer $0 ausgeblendet"
history.state.layerShow "Layer $0 eingeblendet"
history.state.layerSelect "Layer $0 ausgewählt"
history.state.layerUnselect "Layer $0 deselektiert"
history.state.layerName "Layer $0 umbenannt"
history.state.layerFactor "Layer $0 Wert"
history.state.layerFactorOffset "Layer $0 Offset-Wert"
history.state.layerFactorColor "Layer $0 Color-Wert"
history.state.layerFactorRoughness "Layer $0 Roughness-Wert"
history.state.layerFactorMetalness "Layer $0 Metalness-Wert"
// state light
history.state.lightVisible "Licht $0 sichtbar"
history.state.lightIntensity "Licht $0 Stärke"
history.state.lightColor "Licht $0 Farbe"
history.state.lightPosition "Licht $0 Position"
history.state.lightShadow "Licht $0 Schatten"
history.state.lightShadowType ""
history.state.lightShadowBias "Licht $0 Schatten-Bias"
history.state.lightShadowSoftness ""
history.state.lightAttachment "Licht $0 verbunden"
history.state.lightAdd "Licht $0 hinzugefügt"
history.state.lightDelete "Licht $0 gelöscht"
history.state.lightCopy "Licht $0 kopiert"
history.state.lightMove "Licht $0 bewegt"
history.state.lightType "Licht $0 Typ"
history.state.lightSpotAngle "Licht $0 Spot-Winkel"
history.state.lightSpotSoftness "Licht $0 Spot-Santfheit"
// state view
history.state.viewAdd "Ansicht $0 hinzugefügt"
history.state.viewMove "Ansicht $0 Reihenfolge"
history.state.viewDelete "Ansicht $0 gelöscht"

// ----------------------------------------------
// interface
interface "Interface"

// bottom buttons
interface.bottomButtons "Shortcuts hinzufügen (unten)..."

// colors
interface.colors "Hauptfarben"
interface.colorSelect "Widget-Farbe"
interface.colorBase "Grundfarbe"
interface.colorBaseTransparent "Panel-Farbe"
interface.panelTransparent "Panel transparent"
interface.blurFactor "Unschärfe"

// color preset
interface.colorsPresets "Farb-Presets"
interface.presetBlurRed "Rot"
interface.presetBlurBlue "Blau"
interface.presetBlurGreen "Grün"
interface.presetBlurYellow "Gelb"
interface.presetBlackWhite "S/W"
interface.presetWhiteBlack "W/S"
interface.presetLividOrange "Livid & Orange"
interface.presetCardboard "Cardboard"
interface.presetDefault "Standard"

// style
interface.style "Stil"
interface.resetAll "Reset Interface"
interface.resetAll.confirm "Alle Interface-Einstellungen zurücksetzen?"
interface.flipTop "Oben spiegeln"
interface.flipBottom "Unten spiegeln"
interface.flipMiddle "Mitte spiegeln"
interface.showTooltips "Tooltips anzeigen"
interface.showTooltips.help "Dies ist ein Tooltip."
interface.materialPreview "Material-Picker Vorschau"
interface.toolboxHide "Toolbox autom. ausblenden"
interface.toolboxHide.help "Aktivieren Sie diese Option, wenn Sie die Toolbox ausblenden möchten."
interface.toolboxMaxColumn "Toolbox max. Spalten"
interface.toolboxResetOrder "Reset Anordn. Tools"
interface.curveToolSymmetric ""
interface.curveToolSymmetric.help ""
interface.scale "Gesamt-Skalierung"
interface.cursorStep "Vertikale Abstände"
interface.panelWidth "Panel-Breite"
interface.fontScale "Schriftgröße"

// ----------------------------------------------
// layer sub menu
layer.action "Aktion"
layer.name "Name"
layer.delete "Löschen"
layer.move "Bewegen"
layer.duplicate "Duplizieren"
layer.mergeDown "Merge down"
layer.factors "Kanal-Werte"
layer.offsetFactor "Position"
layer.colorFactor "Farbe"

// ----------------------------------------------
// layers menu
layers.addLayer "Layer hinzufügen"
layers.title "Layer"
layers.title.help "Ebenen (Layer) können Positionsverschiebungen und Painting aufzeichnen, was für einen nicht-linearen Arbeitsablauf nützlich sein kann.
Zum Beispiel durch das Experimentieren mit verschiedenen Gesichtsausdrücken, ohne sich auf den Verlauf zurückzugreifen, um die Änderungen rückgängig zu machen.

Beim Painting werden die Ebenen von oben nach unten sortiert, d. h. die obersten Ebenen verdecken die unteren.

Um die Ebenendeckkraft (Layer Opacity) aufzulösen, teilen sich alle Painting-Daten (Color, Roughness, Metalness) die gleiche Maske.
Sie können einen Teil dieser Maske (und damit den Einfluss dieser Ebene) zurücksetzen, indem Sie das Werkzeug 'DelLayer' verwenden."
layers.primitive "Ebenen sind für Grundobjekte (Primitives) nicht verfügbar."
layers.baseSelected "Nichts"

// ----------------------------------------------
// light sub menu
light "Licht"
light.intensity "Stärke"
light.attachment "Anordnung"
light.attachment.fixed "Fixiert"
light.attachment.camera "Kamera"
light.attachment.environment "Environment"
light.attachment.help "-- Fixiert
Die Ausrichtung des Lichts wird sich nicht ändern.

-- Kamera
Die Ausrichtung des Lichts hängt von der Kamera-Ansicht ab."
light.type "Typ"
light.type.directional "Direktional"
light.type.spot "Spot"
light.type.point "Punkt"
light.spotAngle "Winkel Lichtkegel"
light.spotSoftness "Sanftheit"
light.shadowCast "Schatten"
light.shadowType.shadowMap ""
light.shadowType.screenspace ""
light.shadowType.screenspace.help ""
light.shadowBias "Bias"
light.shadowSoftness ""
light.contactShadow ""
light.contactShadow.help ""
light.visible "Zeigen"
light.resetPosition "Zentrieren"

// ----------------------------------------------
// material
material "Material"
material.addNew "Hinzufügen"
// if the shading mode is in matcap or unlit
material.pbrRoughnessMetalnessWarning "Roughness und Metalness werden im aktuellen Shading-Modus irgnoriert."
material.pbrReflectanceWarning ""
material.pbrRefractionWarning ""
material.pbrSubsurfaceWarning ""
// refraction
material.ior "Lichtbrechungsindex (Refraction)"
material.paintingOverride "Painting aufheben"
material.paintingOverride.help ""
material.refractionSurfaceGlossiness "Oberflächenglanz"
material.refractionSurfaceGlossiness.help "- bei 0 nutzt die Oberfläche painted Roughness
- bei 1 ist die Oberfläche völlig glatt"
material.refractionInteriorRoughness "Rauheit innen"
material.refractionInteriorRoughness.help "- bei 0 wird innen painted Roughness benutzt
- bei 1 ist das Innere völlig rau"
material.paintGlossy "Paint Glossy"
material.paintGlossy.help "Painting mit Wert Roughness und Metalness von jeweils 0, für eine eine scharfe Brechung.

Dies entspricht der Nutzung des Paint All-Features aus dem Paint-Menü  mit deaktivierten Color- und Roghness-Kanälen"
// absorption
material.absorptionEnable "Absorption"
material.absorptionEnable.help "Simulieren Sie die Absorption des Lichts, wenn es das Volumen durchbricht.

Dünne Teile werden hell, da sie mehr Licht durchlassen, während dicke Bereiche dunkler sind..

Der Effekt hängt stark von der Mesh-Geometrie ab, es wird nur eine Annäherung an die Mesh-Dicke verwendet."
material.absorptionFactor "Faktor"
// subsurface
material.subsurfaceDepth ""
material.translucency ""
material.translucency.help ""
// type
material.opacity "Opacity"
material.type.opaque "Opaque"
material.type.subsurface ""
material.type.subsurface.help ""
material.type.blending "Blending"
material.type.blending.help ""
material.type.additive "Additive"
material.type.additive.help ""
material.type.dithering "Dithering"
material.type.dithering.help "Das Dithering (Fehlerdiffusion) ist eine Technik um bei Bildern die Illusion einer größeren Farbtiefe zu erzeugen. Dithering ist eine Art des Rasterns."
material.type.refraction "Refraction"
material.type.refraction.help ""
// shadows
material.castShadows "Wirft Schatten"
material.receiveShadows "Empfängt Schatten"
// backface
material.twoSided "Beidseitig"
material.alwaysUnlit "Immer Unlit"
material.flipCulling "Umgekehrtes Culling"
// material
material.reflectance "Reflexionsgrad"
material.reflectance.help "Kontrollieren Sie den Grad der Reflexion, den das Material bei nicht-metallischen Materialien erhält.

In den meisten Fällen sollte der Standardwert verwendet werden (0,5 - was dem Standardwert von 4% reflektiertem Licht bei normalem Winkel entspricht)."

// ----------------------------------------------
// menu name (visible on small screen menu are collapsed)
menu.files "Dateien"
menu.scene "Szene"
menu.multires "Multires"
menu.voxel "Voxel"
menu.dyntopo "Dyntopo"
menu.topology "Deci/UV..."
menu.primitive "Grundformen"
menu.render "Render"
menu.material "Material"
menu.postProcess "Nachbearbeitung"
menu.camera "Kamera"
menu.background "Hintergrund"
menu.tool "Tool"
menu.stroke "Stroke"
menu.paint "Paint"
menu.symmetry "Symmetrie"
menu.pressure "Pressure"
menu.layers "Layer"
menu.settings "Einstellungen"
menu.interface "Interface"
menu.history "Verlauf"
menu.historySettings "Einstellungen"
menu.about "Über"
menu.debug "Debug"

// ----------------------------------------------
// mesh sub menu
mesh.action "Aktion"
mesh.holeClose "Löcher schließen"
mesh.holeDetail "Detail"
mesh.separate "Trennen"
mesh.triplanarWarning "Layer, Painting und Multiresolution geht verloren!"
mesh.triplanarResolution "Auflösung"
mesh.triplanarCubic "Würfelform erzwingen"
mesh.triplanarConvert "Konvertieren"
mesh.name "Name"
mesh.type "Typ"
mesh.typeStatic "Statisch"
mesh.typeMultiresolution "Multiresolution"
mesh.typeDynamic "Dynamisch"

// ----------------------------------------------
// painting
paint.useGlobal "Globales Material"
paint.useGlobal.help "Wenn diese Option aktiviert ist, ist das ausgewählte Material dasselbe wie bei den anderen Werkzeugen.

Beachten Sie, dass hier nur die Einstellungen für Roughness, Metalness und Color berücksichtigt werden."
paint.usePainting "Stroke painting"
paint.intensity "Paint Stäke"
paint.paintAll "Füllen"
paint.paintAll.help "Wendet das aktuelle Material auf das gesamte Mesh ohne maskierte Bereiche an (Paint all).

Maskierte Bereiche und deaktivierte Kanäle werden berücksichtigt und entsprechend ausgespart!"
paint.paintAllForce "Füllen erzwingen"
paint.paintAllForce.help "Wendet das aktuelle Material auf das gesamte Mesh inklusive maskierter Bereiche an (Force paint all).

Maskierte Bereiche und deaktivierte Kanäle werden NICHT berücksichtigt. Es wird wirklich das gesamte Mesch mit dem Material überzogen!"
paint.strokePaintingTitle "Painting ($0)"
paint.layerWarning "Die Kanalmaskierung wird ignoriert, wenn Sie versuchen, sie auf eine Ebene anzuwenden."
paint.texture.title "Textur"
paint.texture.title.help "Ein Bild, das den Brush Stroke färbt.

Beachten Sie, dass Tiling und Skalierung des Alphas genutzt werden."
paint.texture.warningEnable "Stroke Painting muss aktiviert sein, um Texturprojektion zu ermöglichen (Kontrollkästchen oben)!"
paint.texture.warningIgnored "Das aktuelle Werkzeug kann keine Texturen verwenden!"
paint.useAlpha "Stroke Alpha benutzen"
paint.useAlpha.help "Verwenden Sie das Alpha-Set im Stroke-Menu um das Painting zu beeinflussen."
paint.useFalloff "Use stroke falloff"
paint.useFalloff.help "Nutzen Sie das Falloff im Stroke-Menu um das Painting zu beeinflussen."

// ----------------------------------------------
// popup (for example tap on a tool, to open edit popup)
popup.save "Speichern"
popup.save.confirm "Speichern bestätigen?"
popup.lastSave "Letzte Speicher"
popup.lastSave.confirm "Letzte Speicherung laden?"
popup.reset "Reset"
popup.reset.confirm "Zurücksetzen bestätigen?"
popup.clone "Klonen"
popup.rename "Umbenennen"
popup.delete "Löschen"
popup.delete.confirm "Löschen bestätigen?"
popup.delete.confirm.yes "JA, löschen"

// title when requesting input value through virtual keyboard
input.name "Name"
input.number "Wert"
input.hexcolor ""

// ----------------------------------------------
// postprocess
postprocess.mainEnable "Nachbearbeitung (Post)"
postprocess.quality "Qualität"
postprocess.quality.help "Aktivieren Sie diese Optionen, um die Qualität auf Kosten der Leistung zu verbessern.

It will improve:
- Reflektionen
- Ambient Occlusion (AO)
- Tiefenschärfe (DoF - Depth Of Field)"
postprocess.maxSamples "Max. Samples"
postprocess.fullResolution "Volle Auflösung"
postprocess.accumulateCount ""
postprocess.accumulateCount.help ""
postprocess.renderRatio "Render-Auflösung"
postprocess.renderRatioWarning ""
postprocess.renderRatio.help ""
// fxaa
postprocess.fxaaEnable "Anti-Aliasing (FXAA)"
// taa
postprocess.taaEnable "Anti-Aliasing (TAA)"
postprocess.taaEnable.help "Verringert das Flackern, wenn Sie die Kamera bewegen."
// ssr
postprocess.ssrEnable "Reflektion (SSR)"
postprocess.ssrPBRWarning "SSR ist nur im PBR-Shading-Modus wirksam."
// ssao
postprocess.ssaoEnable "Ambient Occlusion (AO)"
postprocess.ssaoRadius "Größe"
postprocess.ssaoFactor "Stärke"
postprocess.ssaoBias "Wölbungs-Bias"
postprocess.ssaoBias.help "Wie empfindlich der Effekt ist, hängt von der Oberflächenwölbung ab."
// dof
postprocess.dofEnable "Depth Of Field (DoF)"
postprocess.dofBlurFar "Ferne Unschärfe"
postprocess.dofBlurNear "Nahe Unschärfe"
postprocess.dofFocusTip "Tippen Sie auf ein Objekt, um den Fokuspunkt zu ändern."
// bloom
postprocess.bloomEnable "Leuchten (Bloom)"
postprocess.bloomIntensity "Stärke"
postprocess.bloomRadius "Radius"
postprocess.bloomRadius.help "Wie ausgedehnt das Leuchten ist."
postprocess.bloomThreshold "Schwellenwert"
postprocess.bloomThreshold.help "Schwellenwert für die Leuchtkraft (Luminosity), um zu entscheiden, ob ein Pixel das Leuchten (Bloom) emittiert oder nicht.
Steht der Wert auf 0, leuchtet alles."
// tone mapping
postprocess.toneEnable "Tone-Mapping"
postprocess.toneExposure "Belichtung"
postprocess.toneContrast "Kontrast"
postprocess.toneSaturation "Sättigung"
postprocess.toneMappingNone "None"
// curve
postprocess.curveEnable "Color-Grading"
postprocess.curve.luminance "Luminanz"
postprocess.curve.red "Rot"
postprocess.curve.green "Grün"
postprocess.curve.blue "Blau"
postprocess.curveReset "Kanal neutral"
postprocess.curveResetAll "Alles neutral"
// chromatic
postprocess.chromaticEnable "Chromatische Aberration"
postprocess.chromaticFactor "Stärke"
// vignette
postprocess.vignetteEnable "Vignette"
postprocess.vignetteSize "Größe"
postprocess.vignetteHardness "Ausprägung"
// sharpness
postprocess.sharpnessEnable "Schärfe"
postprocess.sharpnessFactor "Stärke"
// grain
postprocess.grainEnable "Körnung (Grain)"
postprocess.grainFactor "Stärke"
// curvature
postprocess.curvatureEnable "Curvature"
postprocess.curvatureCavity "Cavity"
postprocess.curvatureBump "Bump"
// pixel art
postprocess.pixelArtEnable ""
// scanline
postprocess.scanlineEnable ""
postprocess.scanlineFactor ""
postprocess.scanlineSpacing ""

// ----------------------------------------------
// primitive (scene menu)
primitive "Grundformen"
primitive.box "Box"
primitive.sphereCube "Sphere"
primitive.sphereUV "UV Sphere"
primitive.icosahedron "Ikosaeder"
primitive.cylinder "Zylinder"
primitive.cone "Kegel"
primitive.torus "Torus"
primitive.lathe "Lathe"
primitive.tube "Tube"
primitive.plane "Plane"
primitive.triplanar "Triplanar"
primitive.faceXYZ ""
primitive.faceXYZ.help ""
primitive.needValidate "Grundformen müssen validiert werden, damit sie für das Sculpting bereit sind."

// for 3d editing in viewport
primitive.useFloatPanel "Panel im Ansichtsfenster"
primitive.useFloatPanel.help "Verschieben Sie einige der Grundform-Optionen direkt ins Ansichtsfenster."
primitive.edit "Edit"
primitive.edit.help "3D-Bearbeitung im Ansichtsfenster zulassen.

Sie können diese Funktion deaktivieren, wenn Sie mit dem Gizmo oder dem Transformieren-Werkzeug arbeiten möchten, um die Grundform zu ändern."

primitive.mainConfig "Parameter"
primitive.topology "Topologie"
primitive.geometry "Geometrie"

// common config
primitive.validate "Validieren"
primitive.maxFaces "Max. Faces"
primitive.maxFaces.help "Die maximale Anzahl an Face (Flächen), die eine Grundform haben kann.

Das Limit ist nur aktiv, solange das Primitiv nicht validiert ist, danach ist der Schutz nicht mehr gegeben."
primitive.linear "Scharfe Kanten"
primitive.subdivision "Post Subdivision"

// common config
primitive.radius "Radius"
primitive.size "Maße"
primitive.sizeX "Größe X"
primitive.sizeY "Größe Y"
primitive.sizeZ "Größe Z"
primitive.division "Untereilung"
primitive.divisionX "Untereilung X"
primitive.divisionY "Untereilung Y"
primitive.divisionZ "Untereilung Z"
primitive.angleX "Winkel X"
primitive.angleY "Winkel Y"
primitive.angleZ "Winkel Z"
primitive.constantDensity "Konstante Dichte"
primitive.projectOnSphere "Auf Sphere projizieren"
primitive.projectOnSphere.help "Rastet die Punkte auf einer perfekten Kugel (Sphere) ein."

// triplanar
primitive.triplanar.title "Triplanar - Einstellung"
primitive.triplanar.title.help "Triplanar verwendet die Maskeninformationen von 3 Ebenen, um ein Voxelgitter zu füllen, das dann in Polygone umgewandelt wird.

Wenn Sie die Regler für Division oder Größe benutzen, werden die Painting-Informationen zurückgesetzt (Smoothness kann verwendet werden).

Sie sollten möglichst Symmetrie deaktivieren, da sie wahrscheinlich nicht so funktioniert, wie Sie es erwarten würden.

Sie können die Option 'Topologisch connected' im Maskenfenster verwenden, um eine Plane zu painten, die die anderen Planes beeinflusst."
primitive.triplanarSameSize "Gleiche Größe (Würfel)"
primitive.triplanarPolish "Smoothness"
primitive.triplanarResetMask "Reset Maske"
primitive.triplanarReproject "Resize Maske"
primitive.triplanarReproject.title "Projizieren Sie die Plane-Maske neu, wenn Sie die triplanaren Einstellungen aktualisieren.

Wenn Sie diese Option deaktivieren, werden wieder die standardmäßigen sphärischen Masken verwendet."
primitive.isolate.all "Alles"
primitive.isolate.back "Hinten"
primitive.isolate.right "Rechts"
primitive.isolate.bottom "Unten"
// plane
primitive.planeSameSize "Gleiche Größe  (quadratisch)"
primitive.planeDisk ""
// box
primitive.boxRegular "Gleiche Größe  (Würfel)"
// tube
primitive.tubeSnapOffset "Snap Offset"
primitive.tubeSnapOffset.help "Ein Wert von 1,0 entspricht dem Radius des Tubes."
primitive.tubeThicknessStart "Start-Radius"
primitive.tubeThicknessEnd "End-Radius"
primitive.tubeTwist ""
primitive.tubeTwistRotate ""
primitive.tubeTwistRadius ""
primitive.tubeTwistOffset ""
primitive.tubeSnap "Snap"
// lathe
// torus
primitive.torusRadiusOuter "Außen-Radius"
primitive.torusRadiusInner "Innen-Radius"
primitive.torusAngle "Winkel"
primitive.torusAngleOffset "Winkel-Offset"
// cylinder
primitive.cylinderHeight "Höhe"
// cone
primitive.coneRadius "Radius"
primitive.coneHeight "Höhe"
// hole sub menu (cylinder, tube, etc)
primitive.hole "Loch"
primitive.hasHole "Hat ein Loch"
// both used for hole radius and main radius
primitive.radiusSync "Gleicher Radius"
primitive.radiusStart "Anfangsradius"
primitive.radiusEnd "Endradius"
primitive.editRadius "Radius"
// spline (for lathe and tube)
primitive.spline "Spline"

// common resources stuffs
resource.delete "Löschen"
resource.import "Import"

// ----------------------------------------------
// scene
scene.title "Szene"
scene.title.help "Wenn Sie das Auswahlkästchen verwenden, halten Sie Ihren Finger gedrückt und ziehen Sie ihn, um andere Objekte einfach auszuwählen."
scene.mergeSimple "Simple Merge"
scene.mergeVoxel "Voxel Merge"
scene.voxelResolution "Auflösung"
scene.subtractionTip "Subtraction  : Mesh ausblenden (Augensymbol)"
scene.intersectionTip "Intersection : Alle Meshes ausgeblendet"

// ----------------------------------------------
// settings
settings.displayTitle "Display Einstellungen"
// wireframe
settings.wireframeTitle "Wireframe"
settings.wireframeDisplay "Wireframe"
settings.debugUV "UV-Checkerboard"
settings.debugUV.help "Zeigt die Wireframe-UV im Hintergrund an, wenn das Modell UVs hat.

Beachten Sie, dass bei Aktivierung dieser Option auch die Anzeige der Checkerboard-Textur erzwungen wird."
// backface
settings.backfaceTitle "Backfaces"
settings.backfaceVisible "Backface"
settings.backfaceVisible.help "Faces sind vomn beiden Seiten sichtbar."
settings.backfaceColor "Backface-Farbe"
settings.backfaceColored "Backfaces einfärben"
// outline
settings.outlineTitle "Umriss"
settings.outlineEnable "Umriss"
settings.outlineThickness "Stärke"
// snap cube
settings.snapCubeTitle "Ansichten-Würfel"
settings.snapCubeDisplay "Ansichten-Würfel"
settings.snapCubeBottom "Unten"
settings.snapCubeLeft "Links"
// stats
settings.statsTitle "Statistik"
settings.statsDisplay "Statistik"
settings.statsRight "Rechts"
settings.statsAll "Gesamte Szene anzeigen"
// grid
settings.gridTitle "Raster (Grid)"
settings.gridDisplay "Raster"
// cursor
settings.cursorWhileSculpting "Kreis anzeigen beim Sculpten"
settings.cursorShowDot "Kleinen Punkt anzeigen"
settings.cursorShowDot.help "Der Punkt kann als Kameradrehpunkt erscheinen oder wenn Sie sculpten."
settings.cursorShowRope "Rope-Stabilisator anzeigen"
// highlight
settings.highlightSelectionTitle ""
settings.highlightSelection ""
settings.highlightDuration ""
// other
settings.darkenUnselected "Nicht gewählte Meshes abdunkeln"
settings.smoothShading "Smooth Shading"
settings.partialDraw "Partial Drawing"
settings.partialDraw.help "Feature noch nicht ausgereift!

Verwenden Sie es, wenn Sie einen relativ kleinen Teil eines High-Poly-Meshes sculpten.

Es sollte das Sculpten snoother machen, aber Sie sollten Wireframe deaktivieren!

Außerdem könnte es bei den Brush-Strokes zu visuellen Artefakten kommen."
settings.partialDrawWarning "Vergessen Sie nicht, diese Option zu deaktivieren, wenn die visuellen Artefakte zu sehr stören!"
settings.showPainting "Pinting anzeigen"
settings.lightIcon "Licht-Icon"
settings.lightIcon.help "Icons für die einzelnen Lichter im Arbeitsbereich anzeigen, so dass Sie die Lichter direkt auswählen und bearbeiten können"
settings.holeTitle "Löcher füllen"
settings.holeNonManifold "Non-Manifold füllen"
settings.holeNonManifold.help "Versucht, ein non-manifold Loch zu füllen.
Diese Option ist nicht in den Einstellungen gespeichert."
settings.loadGuiSettings "GUI-Einstellungen mitladen"
settings.loadGuiSettings.help "Beim Öffnen oder Importieren einer Projektdatei werden alle in das Projekt eingebetteten GUI-bezogenen Einstellungen geladen."
settings.loadObjSplitByGroup "OBJ-Gruppen beibehalten"
settings.loadObjSplitByGroup.help "Wenn diese Option aktiviert ist, teilt Nomad das OBJ in jede Vertex-Gruppe in separate Objekte auf."
settings.loadMergeLayers "Layer zusammenführen"
settings.loadSkipTextures "Texturen überspringen"
settings.loadKeepTopology "Topologie beibehalten"
settings.loadKeepTopology.help "Verwenden Sie diese Option, wenn Sie nicht möchten, dass Nomad die Topologie des importierten Meshes verändert.

Es deaktiviert das Neuanordnen von Vertices/Faces, das Entfernen von Vertex-/Face-Duplikaten und das Entfernen von unbenutzten Vertices."
settings.loadReverseVertices ""
settings.loadReverseVertices.help ""
// multires
settings.multiresTitle "Multiresolution"
settings.multiresMaxVertices "Max. Anzahl an Vertices"
settings.multiresMaxVertices.help "Nomad führt vor der Unterteilung keine Speicherprüfung durch, eine hohe Polyanzahl kann leicht zu Abstürzen führen."
settings.multiresLowResVertices "Low Resolution Schwelle"
settings.multiresLowResVertices.help "Eine geringere Auflösung des Meshes wird angezeigt, wenn Sie die Kamera bewegen.

Sie können diesen Wert erhöhen, wenn Sie eine höhere Auflösung des Meshes anzeigen möchten."

// ----------------------------------------------
// shading
shading "Shading"
// main render mode
shading.pbr "PBR"
shading.pbr.help ""
shading.matcap "MatCap"
shading.matcap.help ""
shading.unlit "Unlit"
shading.unlit.help ""
// textures
shading.textures ""
shading.textures.help ""
// lights
shading.lights "Lichter"
shading.lights.addLight "Licht hinzufügen"
shading.lights.pbrWarning "Lichter werden in den Modi MatCap und Unlit generell ignoriert."
// environment
shading.environment "Umgebung"
shading.environmentImport "HDR importieren"
shading.environmentExposure "Belichtung"
shading.environmentBackgroundBlur ""
shading.environmentRotation "Rotation"
shading.environmentRotation.help "Sie können die Umgebung drehen, indem Sie 3 Finger horizontal (von links nach rechts oder umgekehrt) auf dem Touchbildschirm bewegen."
shading.environmentAttachedToCamera "Mit der Kamera verbunden"
shading.environmentAttachedToCamera.help " Verbinden Sie die Umgebung mit der Kamera.

Dadurch wird eine gleichmäßige Beleuchtung erzwungen, was für das Sculpting hilfreich sein kann."
// matcap
shading.matcap "MatCap"
shading.matcapRotation "Rotation"
shading.matcapRotation.help "Sie können das MatCap drehen, indem Sie 3 Finger horizontal (von links nach rechts oder umgekehrt) auf dem Touchbildschirm bewegen."
shading.matcapGlobal "Globales MatCap verwenden"
shading.matcapGlobal.help "Deaktivieren Sie diese Option, um ein separates MatCap für dieses Mesh zu verwenden."

// ----------------------------------------------
// bottom shortcut buttons (ICON FIT)
shortcut.maskVisible "Maske"
shortcut.solo "Solo"
shortcut.voxelRemesh "Voxel"
shortcut.wireframe "Wire"
shortcut.cameraReset "Reset"
shortcut.cameraSnap "Snap"
shortcut.lockSelection "Lock"
shortcut.perspective "Persp"
shortcut.grid "Raster"
shortcut.uv ""

// can be longer (customization name in Interface menu)
shortcut.voxelRemesh.long "Voxel Remesh"
shortcut.wireframe.long "Wireframe"
shortcut.cameraReset.long "Kamera-Reset"
shortcut.cameraSnap.long "Kamera-Snap"
shortcut.lockSelection.long "Selektion einfrieren"
shortcut.lockSelection.long.help "Wenn diese Funktion aktiviert ist, können Sie die Auswahl nicht ändern, indem Sie auf ein Mesh tippen."
shortcut.perspective.long "Perspektive"
shortcut.grid.long ""

// ----------------------------------------------
// stat
stat.ramScene "Szene"
stat.vramScene "VRAM Szene"
stat.vramRender "VRAM Render"
stat.vramTextures "VRAM Texturen"
stat.ramHistory "Verlauf"
stat.ramOther "Anderes"
stat.usedMemory "RAM benutzt"
stat.freeMemory "RAM frei"
stat.ram "RAM"
stat.used "Benutzt: $0 MB"
stat.free "Frei: $0 MB"
stat.faces "Faces"
stat.triangles "Dreiecke"
stat.vertices "Vertices"
stat.quads "Quads"
stat.sceneFaces "Faces in Szene"
stat.sceneVertices "Vertices in Szene"

// ----------------------------------------------
// stroke
stroke "Stroke"
strokeTitle "Stroke ($0)"
stroke.useWorldRadius "World-Space Radius"
stroke.useWorldRadius.help "Das wird von allen Werkzeugen geteilt."
stroke.useShareRadius "Radius teilen"
stroke.useShareRadius.help "Teilen Sie den Wert des Radius auf mit allen Werkzeugen."
stroke.minSpacingAdjustIntensity "Intensität der Abstände anpassen"
stroke.minSpacingAdjustIntensity.help "Passen Sie die Brush-Intensität an, um eine gleichmäßige Verformung in Abhängigkeit von den Stroke-Abständen zu gewährleisten."
stroke.minSpacing "Stroke Spacing"
stroke.minSpacing.help "Abstand zwischen den einzelnen Strokes (Strichen), relativ zum Werkzeugradius.

Ein niedrigerer Wert ermöglicht einen gleichmäßigeren Stroke, aber die Leistung nimmt ab."
stroke.lazySmooth "Stroke Smoothing"
stroke.lazySmooth.help "Mittlere Position des Pointers, um einen gleichmäßigeren Stroke zu erhalten.

Bei hohen Werten hinkt der Strich dem Pointer hinterher, holt aber schließlich auf."
stroke.lazyRadius "Lazy Rope Stabilisator"
stroke.lazyRadius.help "Die Strokes bleiben in einem bestimmten Abstand hinter der Pointer-Position.

Damit kann man geglättete Linien zeichnen."
stroke.globalSettings "Dies ist eine globale Einstellung"
stroke.snapRadius "Snap Radius"
stroke.snapRadius.help "Rastet den Stroke ein, wenn der Pointer in der Nähe des letzten aufgezeichneten Strokes liegt.

Das kann nützlich sein, wenn man lange, durchgehende Linien zeichnet und dabei häufig Pausen macht."
stroke.sculptOffset "Stroke-Offset"
stroke.sculptOffset.help "Wendet einen konstanten Offset auf den Stroke an.

Diese Option ist für kleine Bildschirme gedacht, wenn man mit den Fingern arbeitet, damit der Finger nicht den Stroke verdecken."
stroke.accumulate "Strokes kumulieren"
stroke.accumulate.help "Wenn diese Option aktiviert ist, gibt es keine Begrenzung für die Menge an Material, die Sie je Stroke hinzufügen/entfernen können."
stroke.useDynamicTopology "Dynamic Topology erlauben"
stroke.connectedTopology "Connected Topology"
stroke.connectedTopology.help "Mit dieser Option werden nur die Vertices gesculptet, die mit der ausgewählten Oberfläche verbunden sind.

Dies wird in der Regel für das Move-Werkzeug verwendet, zum Beispiel, wenn Sie ausschließlich ein Teil verschieben möchten, das sich mit einem anderen Teil überschneidet."
stroke.onlyFrontFace "Nur Front-Facing Vertex"
stroke.onlyFrontFace.help "Diese Option ignoriert Back Face Vertices.

Dies kann nützlich sein, wenn Sie einen Teil einer dünnen Geometrie painten wollen, ohne die andere Seite zu beeinträchtigen.

Dies funktioniert auch für das Sculpting, aber es kann zu Artefakten kommen."
stroke.onlySameSide "Vertices gleicher Ausrichtung"
stroke.onlySameSide.help "Vertices, die in die entgegengesetzte Richtung der Deformation zeigen, werden ignoriert"
stroke.curveFalloff "Falloff"
stroke.onlyLasso "Einstellungen nur für das Lasso-Werkzeug aktiv."
// alpha
stroke.alpha "Alpha"
stroke.alphaInvert "Alpha invertieren"
stroke.alphaWrap "Tiling"
stroke.alphaWrap.none "Kein"
stroke.alphaWrap.repeat "Wiederholen"
stroke.alphaWrap.mirror "Spiegeln"
stroke.alphaProject "Methode"
stroke.alphaProject.surfaceContinuous "Oberfläche"
stroke.alphaProject.screenFixed "Screen Project"
stroke.alphaTiling "Tiling"
stroke.alphaScale "Scaling"
stroke.alphaScale.help "Beim Minimalwert liegt das Alpha-Quadrat innerhalb des Werkzeugkreisradius"
stroke.alphaMidValue "Mittelwert"
stroke.alphaMidValue.help "Mittelwert, bei dem keine Verformung auftritt.

(Mittelwert = 0)
- Schwarz: Kein Displacement
- Weiß: Positives Displacement

(Mittelwert = 0.5)
- Schwarz: Negatives Displacement
- Weiß: Positives Displacement

(Mittelwert = 1)
- Schwarz: Negatives Displacement
- Weiß: Kein Displacement"
// stroke type
stroke.strokeType "Stroke Typen"
stroke.strokeTypeDot "Dot"
stroke.strokeTypeDrag "Drag"
stroke.strokeTypeGrab "Grab"
stroke.strokeTypeGrabRadius "Grab - Dynamischer Radius"
stroke.strokeTypeGrabIntensity "Grab - Dynamische Intensität"

// ----------------------------------------------
// symmetry
symmetry "Symmetrie"
symmetry.enable "Aktiviert"
symmetry.plane.title "Planes"
symmetry.toolIgnore "Das aktuelle Werkzeug ignoriert Symmetrie."
symmetry.radial.title "Radial"
symmetry.radialX "Radial X"
symmetry.radialY "Radial Y"
symmetry.radialZ "Radial Z"
// method
symmetry.method "Methode:"
symmetry.method.help "-- Lokal
Die Symmetrieebene wird entlang des Meshes verschoben, wenn Sie eines der Transformationswerkzeuge (Drehen/Rotate, Verschieben/Translate oder Gizmo) verwenden.


-- Welt
The symmetry plane is fixed and will not move."
symmetry.methodWorld "Welt"
symmetry.methodLocal "Lokal"
// flip
symmetry.flip "Objekt umdrehen"
// mirror
symmetry.mirror "Spiegelung"
symmetry.mirror.help "Versuchen Sie, die Symmetrie wiederherzustellen, ohne die Topologie zu beeinträchtigen.

Radiale Symmetrie wird ignoriert.

Wenn die Topologie nicht beibehalten werden kann, weil sie nicht als symmetrisch angesehen wird, erhalten Sie die Möglichkeit, die Spiegelung dennoch zu erzwingen."
symmetry.mirrorLeftToRight "Von links nach rechts"
symmetry.mirrorRightToLeft "Von rechts nach links"
symmetry.mirrorFail "Versuch die Symmetrie anzuwenden fehlgeschlagen.

Wollen Sie die Symmetrie durch Spiegelung des Meshes erzwingen?"
symmetry.mirrorUseMasking "Maskierte bBereiche schützen"
symmetry.mirrorUseMasking.help "Lässt den maskierten Bereich intakt.

Diese Option wird bei nicht-symmetrischer Topologie (oder unzusammenhängender Oberfläche, wie einem Augenpaar) ignoriert."
// reset
symmetry.reset "Reset"
symmetry.resetCenterMesh "Mesh Zentrum"
symmetry.resetCenterWorld "Welt Zentrum"
symmetry.resetDirection "Orientierung"
// advanced
symmetry.showLine "Line anzeigen"
symmetry.showPlane "Plane anzeigen"
symmetry.editWarning "Symmetriebearbeitung ist experimentell"
symmetry.edit "Gizmo bearbeiten"
symmetry.edit.help "Sie können die Symmetrieebene frei festlegen.

Diese Funktion ist ein wenig experimentell und Sie sollten sie vermutlich nicht verwenden."

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
tool.lassoSelect ""
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
tool.sliderIntensity "Stärke $0 %"

// ----------------------------------------------
// title
tool.settingsTitle "Werkzeugeinstellungen ($0)"

// ----------------------------------------------
// tool menu
tool.noSettings "Dieses Werkzeug hat keine besonderen Einstellmöglichkeiten."

// ----------------------------------------------
// clay
tool.clay.flattenOffset "Flatten Offset"

// ----------------------------------------------
// crease
tool.crease.pinchFactor "Pinch Kraft"

// ----------------------------------------------
// layer
tool.layer.removeInfluence "Aktuellen Layer Offset verwenden"
tool.layer.removeInfluence.help "Diese Option ist nur aktiv, wenn ein aktueller Layer ausgewählt ist.

Es wird der aktuelle Layer Offset verwendet, um das Displacement über Strokes zu begrenzen."
tool.layer.noLayerSelected "Diese Option ist nur verfügbar, wenn ein aktueller Layer ausgewählt ist"

// ----------------------------------------------
// flatten
tool.flatten.warning "Diese Optionen sind experimentell und könnten zukünftig entfallen!"
tool.flatten.planeLockOrigin "Plane Origin sperren"
tool.flatten.planeLockNormal "Plane Ausrichtung sperren"
tool.flatten.planeAverageOrigin "Durchschnittlicher Plane Origin"
tool.flatten.planeAverageNormal "Durchschnittliche Plane Ausrichtung"
tool.flatten.planeOffset "Plane Offset"

// ----------------------------------------------
// smooth
tool.smooth.stickyBorder "Sticky Vertex am Rand"

// ----------------------------------------------
// paint
tool.paint "Paint"
tool.paint.erase "Löschen"
tool.paint.depthFilter "Depth Filter"
tool.paint.layerFilter "Layer Filter"
tool.paint.layerFilter.help "Verwenden Sie diese Option, wenn Sie nur den bereits gepainteten Bereich eines Layers erneut painten wollen."

// ----------------------------------------------
// masking
tool.mask.clear "Löschen"
tool.mask.invert "Umkehren"
tool.mask.flipConnected "Flip Connected"
tool.mask.blur "Blur"
tool.mask.sharpen "Schärfen"
tool.mask.thickness "Shell Dicke"
tool.mask.polish "Rand Smoothness"
tool.mask.engraveEmboss "Gravieren / Prägen"
tool.mask.extract "Extrahieren"
tool.mask.split "Teilen"
tool.mask.closeMask "Closing Aktion (maskiert):"
tool.mask.closeUnmask "Closing Aktion (unmaskiert):"
tool.mask.closeAction "Closing Aktion:"
tool.mask.closeActionNone "Keine"
tool.mask.closeActionFill "Fill"
tool.mask.closeActionShell "Shell"
tool.mask.closeActionLayer "Layer"
tool.mask.closeAction.help "-- Keine
Extrahiert einfach den Bereich und lässt das extrahierte Teil offen.

-- Fill
Das Loch wird gefüllt und geglättet.
Verwenden Sie diese Option nicht für ebene Oberflächen.

-- Shell
Schließt die extrahierte Form mit Hilfe des Dickenwerts.

-- Layer
Extrahiert die Layer-Differenz (nur Layer-Untermenü)."

// ----------------------------------------------
// matrix (transform / gizmo)
tool.matrix "Matrix"
tool.matrix.clone "Klonen"
tool.matrix.action "Aktion"
tool.matrix.action.help "-- Move Origin
Bewegt das Mesh zum Welt Origin (Ursprung).

-- Reset
Setzt die Mesh-Transformation auf Identität zurück.

-- Bake
Wendet die Matrix auf den Vertex an und setzt die Matrix zurück. Visuell sollte sich nichts ändern."
tool.matrix.transformOperation "Transformieren"
tool.matrix.translation "Translation"
tool.matrix.rotation "Rotation"
tool.matrix.scale "Skalierung"
tool.matrix.uniformScale "Gleichmäßige Skalierung"
tool.matrix.uniformScale.help "Nomad kann keine ungleichmäßige Skalierung als Objekttransformation unterstützen, daher wird sie als Vertex-Transformation angewendet."
tool.matrix.moveToOrigin "Move Origin"
tool.matrix.resetTransform "Reset"
tool.matrix.bakeTransform "Backen"
tool.matrix.applyMethod "Methode:"
tool.matrix.applyMethodAuto "Auto"
tool.matrix.applyMethodVertex "Vertex"
tool.matrix.applyMethodObject "Objekt"
tool.matrix.applyMethod.help "-- Auto
Lassen Sie Nomad zwischen dem Vertex- und dem Objektmodus wählen.
Normalerweise wird Objekt bevorzugt, es sei denn, die Symmetrie ist aktiviert oder das Mesh ist maskiert.

-- Vertex
Vertices werden einzeln transformiert.
Symmetrie und Masken werden dabei berücksichtigt.
Für Grundformen, die nicht validiert sind, wird der Objektmodus erzwungen.

-- Objekt
Das Objekt wird als Ganzes transformiert.
Symmetrie und Masken werden ignoriert.
Wenn Sie eine ungleichmäßige Skalierung verwenden, wird der Vertex-Modus erzwungen."

// ----------------------------------------------
// transform
tool.transform.multiTouch "Multi-Touch"
tool.transform.multiTouch.help "Wenn diese Option deaktiviert ist, können Sie jeweils nur einen Modus (Translate, Rotate, Skalieren) verwenden."
tool.transform.transformRestrictRotationY ""
tool.transform.transformRestrictRotationY.help ""

// ----------------------------------------------
// gizmo
tool.gizmo.size "Widget-Größe"
tool.gizmo.linearRollThreshold "Tangentiales Rollen"
tool.gizmo.linearRollThreshold.help " Schwellenwert des Winkels für die Wahl zwischen linearer und kreisförmiger Roll-Methode.

Bei Werten über diesem Schwellenwert wird die kreisförmige Roll-Methode verwendet.

Wenn Sie die lineare Drehung (Richtung der Tangente) bevorzugen, setzen Sie diesen Wert einfach auf 90°."
tool.gizmo.autoHide "Ausblenden bei Interaktion"
tool.gizmo.tap "Pivot Einfach-Tap"
tool.gizmo.tap.help "Diese Option ist nur im benutzerdefinierten Pivot-Modus wirksam (Auto deaktiviert).

-- Keiner
Beim Antippen des Meshes passiert nichts.

-- Erster Treffer
Bewegt das Gizmo auf die erste Schnittstelle.

-- Mittlerer
Bewegt das Gizmo auf den Durchschnitt der ersten beiden Schnittpunkte."
tool.gizmo.tapNone "Keiner"
tool.gizmo.tapFirstHit "Erster Treffer"
tool.gizmo.tapMiddleStab "Mittlerer"

// ----------------------------------------------
// lathe
tool.lathe.axis "Achse"
tool.lathe.axis.fixed "Fixiert"
tool.lathe.axis.dynamic "Dynamisch"

// ----------------------------------------------
// tube
tool.tube.snap "Snapping"
tool.tube.snap.all "Jeder Punkt"
tool.tube.snap.startEnd "Start & Ende"

// ----------------------------------------------
// trim
tool.hole "Füllen von Löchern"
tool.hole.fillHoles "Löcher füllen"
tool.hole.bridges "Screen-Space Boolean"
tool.hole.bridges.help "Wenn diese Option aktiviert ist, können Sie Löcher in das Volumen stechen.
Auch die Schnittneigung wird sich stärker an der Schnittform orientieren."
tool.hole.threshold "Schwellenwert Epsilon"
tool.hole.threshold.help "Eine Optimierung dieses Wertes könnte den Algorithmus zum Füllen von Löchern unterstützen."
tool.hole.smoothing "Loch-Glättung"

// ----------------------------------------------
// smudge
tool.smudge.quality "Qualität"
tool.smudge.quality.help "Es ändert die Auflösung der projizierten Pixel, niedrigere Werte bedeuten schnellere Striche (Strokes)."

// ----------------------------------------------
// trim / split / project / selMask
tool.shape "Shape"
tool.shape.rectangleSquare "Quadrat"
tool.shape.rectangleCentered "Zentriert"
tool.shape.ellipseCircle "Kreis"
tool.shape.ellipseCentered "Zentriert"
tool.shape.lineRotateStep "Rotate Schritt"

// ----------------------------------------------
// measure
tool.measure.goldenRatio "Goldenen Schnitt anzeigen"

// ----------------------------------------------
// topology
topology "Topologie"
// multires
topology.multires.title "Multiresolution"
topology.multires.title.help "Keep multiple resolution of a mesh.

Wenn Sie Änderungen in einer niedrigeren Auflösung vornehmen, werden die Details aus der höheren Auflösung beim Zurückschalten erneut projiziert.

Layer sind in jeder Auflösung verfügbar."
topology.multiresReverse "Reverse"
topology.multiresReverse.confirm "Konnte keine Basis-Subdivision erstellen.

Die derzeitige Topologie ist wahrscheinlich nicht das Ergebnis einer Subdivision"
topology.multiresSubdivide "Subdivide"
topology.multiresSubdivideConfirm "Das Mesh wird $0M Vertices aufweisen, sind Sie sicher?"
topology.multiresDeleteLower "Niedriger löschen"
topology.multiresDeleteHigher "Höher löschen"
topology.multiresKeepTriangles "Dreiecke behalten"
topology.multiresLinear "Linear Subdivision"
topology.multiresLinear.help ""
// voxel
topology.voxel.title "Voxel Remeshing"
topology.voxel.title.help "Remeshing durch Abtasten des Meshes auf einem Raster.

If the object is not closed (watertight), an hole-filling algorithm will be applied first.

Die Layer werden nach dem Remeshing neu projiziert, aber die Qualität verschlechtert sich."
topology.voxelResolution "Auflösung"
topology.voxelRemesh "Remesh"
topology.voxelSharp "Scharfe Kanten beibehalten"
topology.voxelSharp.help "Diese Option ist vor allem für einfache boolesche Operationen nützlich.

Es wird in einigen Bereichen zu Verzerrungen kommen, da die Punkte an den Kanten gefangen werden."
topology.voxelSubLevel "Rebuild Multires"
topology.voxelSubLevel.help "Sie können eine Multiresolution-Hierarchie aus der Voxel-Remesher-Ausgabe wiederherstellen.

Wird schneller ausgeführt und weniger Speicher verbrauchen, besonders wenn der Voxel-Detailwert hoch ist.
Wenn jedoch der Voxel-Detailwert niedrig ist und Sie viele Multires-Level benötigen, werden Sie Details verlieren."
// dynamic topology
topology.surfaceUniform "Remesh"
topology.surfaceDetail "Detail"
topology.surfaceDetail.help "Anders als bei der Voxel-Remeshing-Methode ist es bei der Surface-Remeshing-Methode nicht erforderlich, dass das Mesh geschlossen ist.

Maskierung wird unterstützt, so dass Sie Teile des Meshes mit Masken vor Topologieänderungen schützen können.

Layer werden einwandfrei aktualisiert."
topology.surfaceMethod "Methode"
toplogy.surfaceMethodUniformisation "Uniformisation"
toplogy.surfaceMethodSubdivision "Subdivision"
toplogy.surfaceMethodDecimation "Decimation"
topology.surfaceMethod.help "Das Verhalten der dynamischen Topologie:
- Uniformisation: Hinzufügen und Entfernen von Details
- Subdivision: Details hinzufügen
- Decimation: Details entfernen"
topology.surfaceUseMasking "Maskierte Bereiche schützen"
topology.surfaceUseMasking.help "Die Topologie in den maskierten Bereichen wird von Änderungen ausgenommen."
topology.surfaceExtrapolate "Vertex Extrapolation"
// dynamic
topology.dynamic "Dynamische Topologie"
topology.dynamicActivate "Aktiviert"
topology.dynamicActivate.help "Mit dynamischer Topologie können Sculpting-Werkzeuge das Mesh lokal in Echtzeit unterteilen oder vereinfachen, je nach aktuellem Bedarf.

Diese Funktion kann sich spürbar auf die Leistung auswirken.

Layer werden einwandfrei aktualisiert."
topology.dynamicDetailLevel "Detail"
topology.dynamicDetailEdge "Detail"
topology.dynamicDetailMethod "Detailgrad basierend auf..."
topology.dynamicDetailMethodZoom "Zoom"
topology.dynamicDetailMethodRadius "Radius"
topology.dynamicDetailMethodConstant "Konstant"
topology.dynamicDetailMethod.help "-- Zoom
Der Detailgrad hängt davon ab, wie weit man von der Oberfläche entfernt ist.

-- Radius
Der Radius des Werkzeugs bestimmt den Grad der Detaillierung.

-- Constant
Der Detailgrad ist festgelegt, der Detailwert wird auch mit dem Voxel-Schieberegler geteilt."
topology.dynamicQuality "Bevorzuge..."
topology.dynamicQuality.help "Wenn Sie sich für Qualität entscheiden, sind die 2 wichtigsten Unterschiede:
- die Verfeinerung wird vor dem Sculpting-Operator angewandt, so dass Sie beim Painting oder Sculpting sehr kleiner Details weniger interpolierende Artefakte erhalten
- die Verfeinerung (refinement) wird nicht inkrementell angewandt. Wenn Sie sehr kleine Details modellieren oder schnelle Strokes (Striche) setzen, wird die Topologie immer korrekt verfeinert.

Für eine bessere Leistung und wenn Sie diese Option verwenden möchten, sollten Sie die Option \"partial drawing\" im Einstellungsbereich aktivieren."
topology.dynamicQualitySpeed "Geschwindigkeit"
topology.dynamicQualityQuality "Qualität"
topology.dynamicUsePressure "Druck auf Radius anwenden"
topology.dynamicUsePressure.help "Verwenden Sie diese Option, wenn Sie möchten, dass sich der Stiftdruck (pen pressure) auf den Radius des Werkzeugs auswirkt und die Detailgenauigkeit beeinflusst."
// decimate
topology.decimate.title "Decimation"
topology.decimate.title.help "Verringert die Anzahl der Polygone, wobei versucht wird, so viele Details wie möglich zu erhalten.

Diese Funktion kann nützlich sein, wenn Sie für den 3D-Druck exportieren möchten.
Sie sollten diese Funktion jedoch nicht verwenden, wenn Sie mit Sculpting fortfahren wollen, da sie ungleichmäßige Dreiecke erzeugen kann.

Beachten Sie, dass der maskierte Bereich nicht dezimiert wird."
topology.decimate "Decimate"
topology.decimateTargetFaces "Ziel Dreiecke"
topology.decimatePaintWeight "Painting erhalten"
topology.decimatePaintWeight.help "Ein höherer Wert wird versuchen, Painting zu erhalten.

Setzen Sie diesen Wert auf 0, wenn Painting keine Rolle spielt."
topology.decimateUniform "Uniform Faces"
topology.decimateUniform.help "Ein höherer Wert führt zur Ausgabe von Dreiecken mit ähnlicher Größe."
// topology.decimatePreserveBorders "Preserve borders"
// topology.decimatePreserveBorders.help "Do not decimate the border of the mesh.

// This is only relevant for object that are not watertight."

// BFF is activated through Debug menu
topology.uv.title "UV Auto-Unwrap"
topology.uvAtlas "Unwrap Atlas"
topology.uvAtlas.warning "Kann sehr langsam sein, Ziel: <100k Vertices!"
topology.uvBFF "Unwrap BFF"
topology.uvBFF.warning "Es kann zu Überschneidungen kommen, wenn das Mesh Handles aufweist!"
topology.uvBFFCones "Kegel-Anzahl"
topology.uvBFFCones.help "Ein höherer Wert verringert die Verzerrung bei komplexen Objekten.

Ein höherer Wert bedeutet aber auch eine längere Berechnungszeit."
topology.uvDelete "UVs löschen"

// baking
topology.bake ""
topology.bake.help ""
topology.bakeResolution ""

// ----------------------------------------------
// privacy policy
privacyPolicy.title ""
privacyPolicy.reject ""
privacyPolicy ""

// ----------------------------------------------
// version trial
version.buyWeb "Die Web-Version ist nur eine Demo"
version.buyFull "Upgrade auf Vollversion"
version.restorePurchase "Kauf wiederherstellen"
// version.promoHuawei ""

version.trialHistory "Trial version: Nur 4 UnDo/Redo möglich"
version.trialLayer "Trial version: Nur 1 Layer pro Mesh"
version.trialOneProject "Trial version: Nur 1 aktives Projekt"
version.trialNoImport "Trial version: Kein Import"
version.trialNoExport "Trial version: Kein Export"

version.fullFeatures "- UnDo/ReDo unbegrenzt
- Unbegrenzte Layer
- Speichern  & Laden
- Export & Import"