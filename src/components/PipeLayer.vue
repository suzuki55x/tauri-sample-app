<script>
export default {
  name: "PipeLayer",
  props: {
    pipeData: {
      type: Array,
    },
    options: {
      type: Object,
    },
    layerOptions: {
      type: Object,
    },
  },
  watch: {
    pipeData(newData, oldData) {
      if (newData !== oldData) {
        this.$parent.plot.push(this.layer, newData, this.options);
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

    // start by setting the header of the pipe
    this.layer = this.$parent.plot.overlay_pipe(
      this.options,
      this.layerOptions
    );

    // if data is provided and non-empty, go ahead and
    // begin plotting data
    if (this.pipeData !== undefined && this.pipeData.length > 0) {
      this.$parent.plot.push(this.layer, this.pipeData);
    }
    return null;
  },
  render() {
    return null;
  },
};
</script>
