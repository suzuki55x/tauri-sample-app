<script>
export default {
  name: "ArrayLayer",
  props: {
    plotData: {
      type: Array,
      default() {
        return [];
      },
    },
    options: {
      type: Object,
    },
    layerOptions: {
      type: Object,
    },
  },
  watch: {
    plotData(newData, oldData) {
      if (newData !== oldData) {
        this.$parent.plot.reload(this.layer, newData, this.options);
      }
    },
    options(newOptions, oldOptions) {
      if (newOptions !== oldOptions) {
        this.$parent.plot.headermod(this.layer, newOptions);
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

    this.layer = plot.overlay_array(
      this.plotData,
      this.options,
      this.layerOptions
    );
  },
  render() {
    return null;
  },
};
</script>
