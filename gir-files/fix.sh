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

# Remove GstMemoryEGL and EGLImage
xmlstarlet ed --pf --inplace \
	   --delete '//_:record[@name="GLMemoryEGL"]' \
	   --delete '//_:record[@name="GLMemoryEGLAllocator"]' \
	   --delete '//_:record[@name="GLMemoryEGLAllocatorClass"]' \
	   --delete '//_:record[@name="EGLImage"]' \
	   --delete '//_:record[@name="GLDisplayEGLDeviceClass"]' \
	   --delete '//_:class[@name="GLMemoryEGLAllocator"]' \
	   --delete '//_:class[@name="GLDisplayEGLDevice"]' \
	   --delete '//_:callback[@name="EGLImageDestroyNotify"]' \
	   --delete '//_:constant[@name="GL_MEMORY_EGL_ALLOCATOR_NAME"]' \
	   --delete '//_:function[starts-with(@name, "egl")]' \
	   --delete '//_:function[starts-with(@name, "gl_memory_egl")]' \
	   --delete '//_:function[@name="is_gl_memory_egl"]' \
	   --delete '//_:function-macro[starts-with(@name, "egl")]' \
	   --delete '//_:function-macro[starts-with(@name, "EGL")]' \
	   --delete '//_:function-macro[starts-with(@name, "GL_MEMORY_EGL")]' \
	   --delete '//_:function-macro[starts-with(@name, "IS_EGL_IMAGE")]' \
	   --delete '//_:function-macro[starts-with(@name, "IS_GL_MEMORY_EGL")]' \
	   GstGL-1.0.gir

xmlstarlet ed --pf --inplace \
	   --delete '//_:method[@c:identifier="gst_gl_display_egl_from_gl_display"]' \
	   --delete '//_:method[@c:identifier="egl_from_gl_display"]' \
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
	   --delete '//_:record[@name="VideoAncillary"]/_:field[@name="data"]/_:array/@fixed-size' \
	   --insert '//_:record[@name="VideoAncillary"]/_:field[@name="data"]/_:array' \
              --type attr --name 'fixed-size' --value '256' \
	    GstVideo-1.0.gir
xmlstarlet ed --pf --inplace \
	   --delete '//_:record[@name="ISO639LanguageDescriptor"]/_:field[@name="language"]/_:array/@c:type' \
	   --insert '//_:record[@name="ISO639LanguageDescriptor"]/_:field[@name="language"]/_:array' \
              --type attr --name 'c:type' --value 'gchar' \
	    GstMpegts-1.0.gir

xmlstarlet ed --pf --inplace \
	   --delete '//_:record[@name="MIKEYPayloadKeyData"]/_:field[@name="kv_data"]/_:array/@c:type' \
	   --insert '//_:record[@name="MIKEYPayloadKeyData"]/_:field[@name="kv_data"]/_:array' \
              --type attr --name 'c:type' --value 'guint8' \
	    GstSdp-1.0.gir

# Remove duplicated enums
xmlstarlet ed --pf --inplace \
	   --delete '//_:enumeration[@name="EditMode"]/_:member[starts-with(@name, "edit_")]' \
	   --delete '//_:enumeration[@name="Edge"]/_:member[starts-with(@name, "edge_")]' \
	   GES-1.0.gir

