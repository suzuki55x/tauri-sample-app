import Vue from "vue";
import SigPlot from "./SigPlot";
import ArrayLayer from "./ArrayLayer";
import HrefLayer from "./HrefLayer";
import PipeLayer from "./PipeLayer";

const Components = {
  SigPlot,
  ArrayLayer,
  HrefLayer,
  PipeLayer,
};

Object.keys(Components).forEach((name) => {
  Vue.component(name, Components[name]);
});

export default Components;
