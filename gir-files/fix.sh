#!/bin/bash
set -x -e

# Remove GLFuncs record
# commit 5765641
xmlstarlet ed --pf --inplace --delete '//_:record[@name="GLFuncs"]' GstGL-1.0.gir

# Add a disguised GFuncs record (two steps)
xmlstarlet ed --pf --inplace \
	   --subnode '//_:namespace' --type elem -n 'recordTMP' --value ' ' \
	   GstGL-1.0.gir

xmlstarlet ed --pf --inplace \
	   --insert '//_:recordTMP' -t attr -n 'name' --value 'GLFuncs' \
	   --insert '//_:recordTMP' -t attr -n 'c:type' --value 'GstGLFuncs' \
	   --insert '//_:recordTMP' -t attr -n 'disguised' --value '1' \
	   --rename '//_:recordTMP' --value 'record' \
	   GstGL-1.0.gir

# incorrect GIR due bug #797144
xmlstarlet ed --pf --inplace \
	   --update '//*[@c:identifier="Dubois optimised Green-Magenta anaglyph"]/@c:identifier' \
	     --value GST_GL_STEREO_DOWNMIX_ANAGLYPH_GREEN_MAGENTA_DUBOIS \
	   --update '//*[@c:identifier="Dubois optimised Red-Cyan anaglyph"]/@c:identifier' \
	     --value GST_GL_STEREO_DOWNMIX_ANAGLYPH_RED_CYAN_DUBOIS \
	   --update '//*[@c:identifier="Dubois optimised Amber-Blue anaglyph"]/@c:identifier' \
	      --value GST_GL_STEREO_DOWNMIX_ANAGLYPH_AMBER_BLUE_DUBOIS \
	   GstGL-1.0.gir

# Remove GstDisplayWayland
xmlstarlet ed --pf --inplace \
	   --delete '//_:class[@name="GLDisplayWayland"]' \
	   --delete '//_:record[@name="GLDisplayWaylandClass"]' \
	   GstGL-1.0.gir

# Remove GstDisplayX11
xmlstarlet ed --pf --inplace \
	   --delete '//_:class[@name="GLDisplayX11"]' \
	   --delete '//_:record[@name="GLDisplayX11Class"]' \
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
