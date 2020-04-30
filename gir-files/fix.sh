#!/bin/bash
set -x -e

# https://github.com/gtk-rs/gir-files/blob/master/reformat.sh
# `///` used as `//` not works in Windows in this case
for file in *.gir; do
	xmlstarlet ed -P -L \
		-d '//_:doc/@line' \
		-d '//_:doc/@filename' \
		-d '///_:source-position' \
		"$file"
done

# replace wayland structures to gpointers
xmlstarlet ed --pf --inplace \
            --update '//*[@c:type="wl_display*"]/@c:type' \
              --value gpointer \
	    --update '//*[@c:type="wl_registry*"]/@c:type' \
	      --value gpointer \
	    --update '//*[@c:type="wl_compositor*"]/@c:type' \
	      --value gpointer \
	    --update '//*[@c:type="wl_subcompositor*"]/@c:type' \
	      --value gpointer \
	    --update '//*[@c:type="wl_shell*"]/@c:type' \
	      --value gpointer \
	    GstGL-1.0.gir

# Change X11's Display* and xcb_connection_t* pointers to gpointer
xmlstarlet ed --pf --inplace \
	   --insert '//_:type[@c:type="Display*"]' \
              --type attr --name 'name' --value 'gpointer' \
	   --insert '//_:type[@c:type="xcb_connection_t*"]' \
              --type attr --name 'name' --value 'gpointer' \
            --update '//*[@c:type="Display*"]/@c:type' \
              --value gpointer \
	    --update '//*[@c:type="xcb_connection_t*"]/@c:type' \
	      --value gpointer \
	    GstGL-1.0.gir

# Remove GstMemoryEGL
xmlstarlet ed --pf --inplace \
	   --delete '//_:record[@name="GLMemoryEGL"]' \
	   --delete '//_:record[@name="GLMemoryEGLAllocator"]' \
	   --delete '//_:record[@name="GLMemoryEGLAllocatorClass"]' \
	   GstGL-1.0.gir

xmlstarlet ed --pf --inplace \
	   --delete '//_:method[@c:identifier="gst_gl_display_egl_from_gl_display"]' \
	   GstGL-1.0.gir

# Remove all libcheck related API
xmlstarlet ed --pf --inplace \
	   --delete '//_:function[starts-with(@name, "check_")]' \
	   --delete '//_:function[starts-with(@name, "buffer_straw_")]' \
	   --delete '//_:callback[starts-with(@name, "Check")]' \
	   --delete '//_:record[starts-with(@name, "Check")]' \
	   GstCheck-1.0.gir

# Change GstVideoAncillary.data to a fixed-size 256 byte array
xmlstarlet ed --pf --inplace \
	   --delete '//_:record[@name="VideoAncillary"]/_:field[@name="data"]/_:array/@length' \
	   --insert '//_:record[@name="VideoAncillary"]/_:field[@name="data"]/_:array' \
              --type attr --name 'fixed-size' --value '256' \
	    GstVideo-1.0.gir

