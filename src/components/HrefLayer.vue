<script>
export default {
  name: "ArrayLayer",
  props: {
    href: {
      type: String,
    },
    onload: {
      type: Function,
    },
    layerOptions: {
      type: Object,
    },
  },
  watch: {
    href(newHref, oldHref) {
      if (newHref !== oldHref) {
        this.$parent.plot.deoverlay(this.layer);
        this.$parent.plot.overlay_href(newHref, this.onload, this.layerOptions);
      }
    },
    layerOptions(newLayerOptions, oldLayerOptions) {
      if (newLayerOptions !== oldLayerOptions) {
        this.$parent.plot.get_layer(this.layer).change_settings(newLayerOptions);
      }
    },
  },
  created() {
    const plot = this.$parent.plot;
    if (!plot) {
      return;
    }

    this.layer = plot.overlay_href(this.href, this.onload, this.layerOptions);
  },
  render() {
    return null;
  },
};
</script>
